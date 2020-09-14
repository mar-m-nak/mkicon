use load_file::load_bytes;
use std::env;
use tinybmp::{Bmp, Pixel};

fn read_bmp(path: &str) {
    // 外部ファイル読み込み
    let bmp = Bmp::from_slice(load_bytes!(path)).expect("Failed to parse BMP image");

    // 画像サイズ, bpp, 総ピクセル数 でチェック
    assert_eq!(bmp.header.image_width, 16);
    assert_eq!(bmp.header.image_height, 16);
    assert_eq!(
        true,
        bmp.header.bpp == 8 || bmp.header.bpp == 16 || bmp.header.bpp == 24 || bmp.header.bpp == 32
    );

    // BMPのピクセル座標と色のイテレータを取得し vec に収集
    let pixels: Vec<Pixel> = bmp.into_iter().collect();
    assert_eq!(pixels.len(), 16 * 16);

    println!(
        "- size : w {}, h {}",
        bmp.header.image_width, bmp.header.image_height
    );
    println!("- bpp : {}", bmp.header.bpp);

    // 全灯色判定（パレット番号1 or 白 を全灯色bitと判定）
    //  bpp  8bit ... 1（パレット番号）
    //  bpp 16bit ... 32767
    //  bpp 24bit ... 16777215
    //  bpp 32bit ... 16777215
    let lights_color =
        if bmp.header.bpp == 16 {32767} else if bmp.header.bpp >= 24 {16777215} else {1};

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
    for ptn in &lights_patterns {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("");
    for ptn in &harf_patterns {
        print!(" {}", format!("{:04X}", ptn));
    }
}

fn main() {
    println!("BMP to monoicon pattern for CatShanty2");
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("{:?}", path);
    read_bmp(path);
}
