use load_file::load_bytes;
use std::env;
use tinybmp::{Bmp, Pixel};

/**
 * カレントディレクトリを返す
 */
fn get_curdir() -> String {
    let curres = env::current_dir().expect("カレントディレクトリ取得失敗");
    curres.display().to_string()
}

/**
 * BMPファイルを読み込んで CatShanty2 のモノアイコン用bitパターンを表示する
 */
fn read_bmp(path: &str) {

    // path指定無ければカレントディレクトリ上を指定
    let curdir = get_curdir();
    let read_path: String = if path.contains(":") || path.contains("\\") || path.contains("/") {
        path.to_string()
    } else {
        format!("{}\\{}", curdir, path)
    };
    println!("読み込みファイル : \"{}\"", read_path);

    // 外部ファイル読み込み
    let bmp = Bmp::from_slice(load_bytes!(&read_path)).expect("BMPファイル展開失敗");

    // 画像サイズ, bpp, 総ピクセル数 でチェック
    assert_eq!(
        true,
        bmp.header.image_width == 16 || bmp.header.image_height == 16,
        "規定外のサイズです"
    );
    assert_eq!(
        true,
        bmp.header.bpp == 8 || bmp.header.bpp == 16 || bmp.header.bpp == 24 || bmp.header.bpp == 32,
        "対応していない色深度です"
    );
    println!("色深度 : {}bpp", bmp.header.bpp);

    // BMPのピクセル座標と色のイテレータを取得し vec に収集
    let mut pixels: Vec<Pixel> = bmp.into_iter().collect();
    assert_eq!(pixels.len(), 16 * 16, "ピクセル取得失敗");


    // 全灯色判定（パレット番号1 or 白 を全灯色bitと判定）
    //  bpp  8bit ... 1（パレット番号）
    //  bpp 16bit ... 32767
    //  bpp 24bit ... 16777215
    //  bpp 32bit ... 16777215
    let lights_color = if bmp.header.bpp == 16 {
        32767
    } else if bmp.header.bpp >= 24 {
        16777215
    } else {
        1
    };

    // 結果bitパターン配列作成
    let mut lights_ptns: [u16; 16] = [0; 16];
    let mut harf_ptns: [u16; 16] = [0; 16];
    while pixels.len() > 0 {
        // １ピクセル取り出し：右下→左上座標
        //  パレット番号0 or 黒 はスキップする
        let pixel = pixels.pop().unwrap();
        if pixel.color == 0 {
            continue;
        }
        // 結果配列の該当bitを立てる
        //  全灯色以外は中間色と判定する
        let bitlfg: u16 = 1 << (15 - pixel.x);
        let py = pixel.y as usize;
        if pixel.color == lights_color {
            lights_ptns[py] |= bitlfg;
        } else {
            harf_ptns[py] |= bitlfg;
        }
    }

    // 結果表示
    println!("\n---モノアイコンパターン ここから---------");
    for ptn in &lights_ptns {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("");
    for ptn in &harf_ptns {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("\n---モノアイコンパターン ここまで---------");
}

fn main() {
    const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("");
        read_bmp(&args[1]);
    } else {
        println!("\n{} {}", PKG_NAME, PKG_VERSION);
        println!("BMP ファイルから CatShanty2 のモノアイコンパターンを作成します.");
        println!("使い方：mkicon \"bmp file\"\n");
        println!("ドットパターンは 背景色, 全灯色, 中間色 と判定した３色で拾います.");
        println!("- 背景色: パレット番号 0 (8bpp) または 黒色 (16,24,32bpp) の bit");
        println!("- 全灯色: パレット番号 1 (8bpp) または 白色 (16,24,32bpp) の bit");
        println!("- 中間色: それ以外の bit");
    }
}
