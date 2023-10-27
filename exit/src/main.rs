use pcsc::{Context, Protocols, Scope, ShareMode, MAX_BUFFER_SIZE};
use std::{
    env::args,
    ffi::CStr,
    fs::OpenOptions,
    io::{BufWriter, Write},
    net::Ipv4Addr,
    str::FromStr,
    sync::{Mutex, OnceLock},
    thread,
    time::Duration,
};

static mut READERS_BUF: [u8; 2048] = [0; 2048];
static IDM_BEFORE: Mutex<u64> = Mutex::new(0);
static IP_ADDRESS: OnceLock<Ipv4Addr> = OnceLock::new();

fn main() {
    let ip_arg = args().collect::<Vec<_>>();
    let ip_arg = ip_arg.get(1).map_or_else(
        || {
            println!("IPアドレスもらってないのでデフォルトの127.0.0.1にしますよ！");
            String::from("127.0.0.1")
        },
        |x| x.to_owned(),
    );
    IP_ADDRESS
        .get_or_init(|| Ipv4Addr::from_str(&ip_arg).expect("有効なIPアドレスじゃないですよー"));
    let context = Context::establish(Scope::User).unwrap();
    let mut readers = context.list_readers(unsafe { &mut READERS_BUF }).unwrap();
    let reader1 = readers.next();
    let reader2 = readers.next();
    println!("使用するカードリーダ: {reader1:?}");
    println!("使用するカードリーダ: {reader2:?}");
    let _ = context.release();
    let thread1;
    let thread2;
    if let Some(reader1) = reader1 {
        thread1 = thread::spawn(move || exit(reader1));
    } else {
        thread1 = thread::spawn(|| {});
    }
    if let Some(reader2) = reader2 {
        thread2 = thread::spawn(move || exit(reader2));
    } else {
        thread2 = thread::spawn(move || {});
    }
    let _ = thread1.join();
    let _ = thread2.join();
}

fn exit(reader: &CStr) {
    loop {
        let context = Context::establish(Scope::User).unwrap();

        let Ok(card) = context.connect(reader, ShareMode::Shared, Protocols::ANY) else {
            thread::sleep(Duration::from_millis(300));
            continue;
        };

        //読み取り完了後の処理
        let apdu = [0xFF, 0xCA, 0x00, 0x00, 0x00];
        let mut rapdu_buf = [0; MAX_BUFFER_SIZE];

        let Ok(rapdu) = card.transmit(&apdu, &mut rapdu_buf) else {
            continue;
        };

        let len = rapdu.len();
        let result = &rapdu[len - 2..len];

        if !(*result.get(0).unwrap() == 0x90 && *result.get(1).unwrap() == 0x00) {
            eprintln!("IDmの読み出しが失敗したよー")
        } else {
            let idm = &rapdu[..len - 2];
            let Ok(idm_bytes) = idm.try_into() else {
                eprintln!("IDmが64bitじゃないよー");
                thread::sleep(Duration::from_millis(300));
                continue;
            };
            let idm = u64::from_be_bytes(idm_bytes);
            if idm != *IDM_BEFORE.lock().expect("Mutexのエラーは考えてない") {
                *IDM_BEFORE.lock().expect("Mutexのエラーは考えてない") = idm;
                let url = format!(
                    "http://{}/exit.php?idm={}",
                    IP_ADDRESS.get().expect("さすがにここでエラーは出ない。ここは初期化されてることが保証されているはず"),
                    idm
                );
                reqwest::blocking::get(url).map_or_else(
                    |_| {
                        eprintln!("ネットワークエラーあるよ");
                    },
                    |_| {},
                )
            }
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("exit.txt")
                .unwrap_or_else(|e| panic!("ファイル作成の時点でエラー起きたんやけど:{}", e));
            let mut buff = BufWriter::new(file);
            buff.write(format!("{}\nexit\n", idm).as_bytes())
                .unwrap_or_else(|e| {
                    eprintln!("ファイル書き込みエラーって何??? :{}", e);
                    0
                });
            buff.flush().unwrap_or_else(|e| {
                eprintln!("フラッシュ失敗した(さすがになくね？): {}", e);
            });
        }

        let _ = context.release();
        thread::sleep(Duration::from_millis(300));
    }
}
