
use pcsc::{Context, Protocols, Scope, ShareMode, MAX_BUFFER_SIZE};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    thread,
    time::Duration,
};
fn main() {
    const N: usize = 200;

    let context = Context::establish(Scope::User).unwrap();
    let mut readers_buf = [0; 2048];
    let mut readers = context.list_readers(&mut readers_buf).unwrap();
    let mut idm_before = 0u64;
    let reader = readers.nth(0).unwrap();
    println!("使用するカードリーダ: {reader:?}");
    let _ = context.release();
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
            eprintln!("IDmの読み出しに失敗しました。")
        } else {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("IDm.txt")
                .unwrap_or_else(|e| panic!("ファイル作成の時点でエラー起きたんやけど:{}", e));
            let mut buff = BufWriter::new(file);

            let idm = &rapdu[..len - 2];
            let idm = u64::from_be_bytes(idm.try_into().expect("え？IDm64bitじゃないの?"));

            if idm != idm_before {
                idm_before = idm;

                let url = "http://127.0.0.1/exit.php?";
                let url =  url.to_string() + "idm=" + &idm.to_string();
                println!("{url}");
                let body = reqwest::blocking::get(url).expect("ネットワークerror");
                
                
                //request
            }
            buff.write(idm.to_string().as_bytes())
                .unwrap_or_else(|e| panic!("??? :{}", e));
            buff.flush()
                .unwrap_or_else(|e| panic!("フラッシュ失敗した(さすがになくね？): {}", e));
        }

        let _ = context.release();
        thread::sleep(Duration::from_millis(300));
    }
}
