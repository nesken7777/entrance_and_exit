
use pcsc::{Context, Protocols, Scope, ShareMode, MAX_BUFFER_SIZE};
use rand::prelude::*;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    thread,
    time::Duration,
};
fn main() {
    const N: usize = 200;
    let array1 = [
        "伝説の",
        "かけだしの",
        "さすらいの",
        "見習いの",
        "今日から私は",
        "生意気な",
        "憧れの",
        "徹夜漬けの",
        "悟りを開いた",
        "虚無の",
        "世界の",
        "経験豊富な",
        "桁外れな",
        "一線を越えた",
        "武を極めた",
        "限界を越えし",
        "ピカピカの",
        "口先だけの",
        "昭和の",
        "平成の",
        "令和の",
        "シャイな",
        "そうです私が",
        "ほとんど",
        "四捨五入したら",
        "単位を落とした",
        "夢見る",
        "安全第一の",
        "地元最強の",
        "前向きな",
        "宇宙最強の",
        "未来を見据える",
        "親戚の",
        "場数を踏んだ",
        "無敗の",
        "留年した",
        "退部した",
        "目立ちたがる",
        "地球にやさしい",
        "天下無敗の",
        "気づいたら",
        "生まれたての",
        "実力派",
        "国民的",
        "クレイジーな",
        "百発百中の",
        "瞬殺の",
        "野性的な",
        "怪物級の",
        "奇跡を起こす",
        "未来から来た",
        "偉大なる",
        "おしゃれな",
        "いつも笑顔の",
        "立ち上がれ!",
        "都会の",
        "田舎の",
        "絶☆対☆的",
        "グローバルな",
        "みんなの",
        "人気者の",
        "お隣の",
        "聖なる",
        "SNS界の",
        "社交的な",
        "不思議の国の",
        "時を駆ける",
        "反逆の",
        "れっきとした",
        "ご存知の通り",
        "慈悲深き",
        "噂の",
        "半端ない",
        "クセの強い",
        "涙目の",
        "幻の",
        "ちょっとした",
        "崖の上の",
        "おしゃべりな",
        "四皇",
        "麦わらの",
        "魔法少女っぽい",
        "百獣の",
        "霊長類最強",
        "冬を乗り越えた",
        "鬼がかった",
        "天才的な",
        "秘密の多い",
        "転生した",
        "心を読む",
        "ギネスに認められた",
        "進撃の",
        "陽気な",
        "頭の良い",
        "正義の",
        "センスのある",
        "可愛い",
        "かっこいい",
        "実は私が",
        "死にかけの",
        "微分された",
        "積分された",
        "ストレスの溜まった",
        "音速の",
        "控えめな",
        "そろそろ本気を出す",
        "記憶をなくした",
        "心配性の",
        "中国産の",
        "ツンデレな",
        "メンヘラな",
        "前の席の",
        "家出した",
        "岐阜高専の",
    ];
    let array2 = [
        "落ち武者",
        "コンデンサ",
        "ゲーマー",
        "留年生",
        "先輩",
        "王様",
        "番長",
        "ヤンキー",
        "スパイ",
        "呪術師",
        "神",
        "火星人",
        "ルーキー",
        "自称ニート",
        "リア充",
        "自宅警備員",
        "秘密兵器",
        "勝負師",
        "問題児",
        "刑事",
        "オタク",
        "課長",
        "部長",
        "永世名人",
        "英雄",
        "当たり屋",
        "スナイパー",
        "子犬",
        "ギャル",
        "マッチョ",
        "子猫",
        "大魔導士",
        "人間国宝",
        "レジェンド",
        "寂しがり屋",
        "YESマン",
        "アイドル",
        "セレブ",
        "大富豪",
        "サラブレッド",
        "女王",
        "乙女",
        "堕天使",
        "熾天使",
        "妖精",
        "騎士",
        "ガリ勉",
        "江戸っ子",
        "努力家",
        "殿様",
        "ならず者",
        "イケメン",
        "キャリアウーマン",
        "猛者",
        "コレクター",
        "先生",
        "教授",
        "博士",
        "クソメガネ",
        "料理人",
        "芸術家",
        "医者",
        "警察官",
        "作家",
        "エンジニア",
        "スポーツ選手",
        "政治家",
        "シェフ",
        "バーテンダー",
        "バンドマン",
        "美容師",
        "歌手",
        "ダンサー",
        "インフルエンサー",
        "プログラマー",
        "軍人",
        "総理大臣",
        "リーダー",
        "学級委員長",
        "生徒会長",
        "サラリーマン",
        "心理学者",
        "コメディアン",
        "考古学者",
        "海賊王",
        "忍者",
        "ゾンビ",
        "トトロ",
        "特級呪物",
        "ラスボス",
        "ヒーロー",
        "現実主義者",
        "天才子役",
        "花子さん",
        "巨人",
        "勇者",
        "ゴブリン",
        "お化け",
        "ドラゴン",
        "スライム",
        "文豪",
        "鬼殺隊",
        "高専生",
        "ダイオード",
        "抵抗器",
        "CPU",
        "キーボード",
        "ラッパー",
        "セミコロン",
        "ハローワールド",
        "リスト",
        "関数",
        "ハードウェア",
        "ソフトウェア",
        "ビット",
        "バイト",
        "磁場",
        "電子",
        "コイル",
        "スイッチ",
        "ネットワーク",
        "電界",
        "磁力線",
        "暗号",
        "点P",
        "トランジスタ",
    ];
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
                let mut rng = thread_rng();
                let x = rng.gen_range(0..112);
                let y = rng.gen_range(0..124);
                let name =String::from(array1[x])+ array2[y] ;
                println!("{name:?}");

                let url = "http://127.0.0.1/name_insert.php?name=";
                let url =  url.to_string()+ &name  + "&idm=" + &idm.to_string();
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
