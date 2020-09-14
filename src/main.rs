use load_file::load_bytes;
use std::env;
use tinybmp::{Bmp, Pixel};

/**
 * カレントディレクトリを返す
 */
fn get_curdir() -> String {
    let curres = env::current_dir().expect("カレントディレクトリ取得失敗");
    return curres.display().to_string();
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
    let pixels: Vec<Pixel> = bmp.into_iter().collect();
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

    // 結果bitパターン格納配列
    let mut lights_patterns: [u16; 16] = [0; 16];
    let mut harf_patterns: [u16; 16] = [0; 16];

    // 16 行
    for py in (0..256).step_by(16) {
        // 16 ピクセル分のbitパターンを作成する
        //  パレット番号0 or 黒 はスキップする
        //  全灯色以外は中間色と判定する
        let mut lights_pttern: u16 = 0;
        let mut harf_pattern: u16 = 0;
        let mut pos: usize = 0;
        for px in (0..16).rev() {
            pos = py + px;
            if pixels[pos].color == 0 {
                continue;
            }
            let bitflg: u16 = 1 << (15 - px);
            if pixels[pos].color == lights_color {
                lights_pttern = lights_pttern | bitflg;
            } else {
                harf_pattern = harf_pattern | bitflg;
            }
        }
        // 結果配列へ格納
        lights_patterns[pos / 16] = lights_pttern;
        harf_patterns[pos / 16] = harf_pattern;
    }

    // 結果表示
    println!("\n---モノアイコンパターン ここから---------");
    for ptn in &lights_patterns {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("");
    for ptn in &harf_patterns {
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
