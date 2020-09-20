use load_file::load_bytes;
use std::env;
use tinybmp::{Bmp, Pixel};

struct MyBmpDatas {
    bpp: u16,
    pixels: Vec<Pixel>,
}
struct BitsPatterns {
    lights: [u16; 16],
    harf: [u16; 16],
}


/**
 * カレントディレクトリを返す
 */
fn get_curdir() -> String {
    let curres = env::current_dir().expect("カレントディレクトリ取得失敗");
    curres.display().to_string()
}

/**
 * BMPファイルを読み込んで bpp と ピクセルのベクタを返す
 */
fn read_bmp(path: &str) -> MyBmpDatas {
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

    MyBmpDatas {bpp: bmp.header.bpp, pixels}
}

/**
 * bpp と ピクセルのベクタから、全灯色 と 中間色 のパターン配列を返す
 */
fn make_bit_pattern(bpp_and_pixels: MyBmpDatas) -> BitsPatterns {
    let bpp = bpp_and_pixels.bpp;
    let mut pixels = bpp_and_pixels.pixels;

    // 全灯色判定（パレット番号1 or 白 を全灯色bitと判定）
    //     8 bpp ... 0x1 (パレット番号)
    //    16 bpp ... 0x8000 (32767)
    // 24/32 bpp ... 0xFF FFFF (16777215)
    let lights_color = if bpp == 16 {0x8000} else if bpp >= 24 {0xFF_FFFF} else {1};

    // 結果bitパターン配列作成
    // パレット番号0 or 黒 はスキップ、全灯色以外は中間色と判定
    let mut lights_ptns: [u16; 16] = [0; 16];
    let mut harf_ptns: [u16; 16] = [0; 16];
    while pixels.len() > 0 {
        // １ピクセル取り出し：座標的には右下→左上に向かう
        let pixel = pixels.pop().unwrap();
        if pixel.color == 0 {
            continue;
        }
        let bitlfg: u16 = 1 << (15 - pixel.x);
        if pixel.color == lights_color {
            lights_ptns[pixel.y as usize] |= bitlfg;
        } else {
            harf_ptns[pixel.y as usize] |= bitlfg;
        }
    }

    BitsPatterns {lights: lights_ptns, harf: harf_ptns}
}

/**
 * パターン配列の結果を表示する
 */
fn disp_result(patterns: BitsPatterns) {
    // 結果表示
    println!("\n---モノアイコンパターン ここから---------");
    for ptn in &patterns.lights {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("");
    for ptn in &patterns.harf {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("\n---モノアイコンパターン ここまで---------");
}

/**
 * BMPファイルを読み込んで CatShanty2 のモノアイコン用bitパターンを表示する
 */
fn main() {
    const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("");
        let bpp_and_pixels = read_bmp(&args[1]);
        let patterns = make_bit_pattern(bpp_and_pixels);
        disp_result(patterns);
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

#[cfg(test)]
mod tests {
    use super::read_bmp;
    use super::make_bit_pattern;

    /**
     * clip.bmp でパターン作成テスト
     */
    #[test]
    fn make_clip_bmp_pattern() {
        let bpp_and_pixels = read_bmp("../tests/ren_clip.bmp");
        let patterns = make_bit_pattern(bpp_and_pixels);
        let pat_0: [u16; 16] = [
            0x0000, 0x0C00, 0x1200, 0x2100, 0x2480, 0x1240, 0x4920, 0x2490, 0x1248, 0x0924, 0x0494, 0x0264, 0x0108, 0x00F0, 0x0000, 0x0000,
            ];
        let pat_1: [u16; 16] = [
                0x0000, 0x1000, 0x2400, 0x0200, 0x0100, 0x2480, 0x1240, 0x4920, 0x2490, 0x1248, 0x0920, 0x0480, 0x0204, 0x0108, 0x0000, 0x0000,
        ];
        assert_eq!(patterns.lights, pat_0, "\n全灯色パターン作成失敗!!\n\n");
        assert_eq!(patterns.harf, pat_1, "\n中間色パターン作成失敗!!\n\n");
    }
}
