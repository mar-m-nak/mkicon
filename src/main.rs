//! mkicon
//!
//! BMP ファイルから CatShanty2 モノアイコン用パターンを生成して表示します
#![warn(missing_docs)]

use std::{fs::File};
use std::io::Read;
use std::env;

// tinybmp を Apache ライセンス 2.0 で使用します
// https://crates.io/crates/tinybmp
use tinybmp::{Bmp, Pixel};

///
/// tinybmp から必要な情報を受け取る構造体
///
struct MyBmpDatas {
    /// 色深度
    bpp: u16,
    /// ピクセル配列
    pixels: Vec<Pixel>,
}

impl MyBmpDatas {
    ///
    /// BMPファイルから必要な情報を取り出す
    ///
    /// BMPファイルを読み込んで [bppとピクセルのベクタ情報](struct.MyBmpDatas.html)を返す
    ///
    /// #Panics
    /// - BMPファイルとして展開出来なかった場合
    /// - 16x16ピクセル以外
    /// - 8,16,24,32bpp 以外の色深度
    /// - 取得したピクセル数が16x16と不一致
    ///
    fn load(path: &str) -> Result<Self, String> {
        // 外部ファイル読み込み
        println!("読み込みファイル : \"{}\"", path);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(String::from("ファイルが開けません")),
        };
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return Err(String::from("バッファオーバーフロー"));
        };
        let bmp = match Bmp::from_slice(&buffer) {
            Ok(bmp) => bmp,
            Err(_) => return Err(String::from("BMPファイル展開失敗")),
        };
        println!("ピクセルサイズ : {} x {}", bmp.header.image_width, bmp.header.image_height);
        if bmp.header.image_width != 16 || bmp.header.image_height != 16 {
            return Err(String::from("対応していないピクセルサイズです(16x16固定)"));
        }
        println!("色深度 : {} bpp", bmp.header.bpp);
        if bmp.header.bpp != 8 && bmp.header.bpp != 16 && bmp.header.bpp != 24 && bmp.header.bpp != 32 {
            return Err(String::from("対応していない色深度です"));
        }
        // BMPのピクセル座標と色のイテレータを取得し vec に収集
        let pixels: Vec<Pixel> = bmp.into_iter().collect();
        if pixels.len() != 16 * 16 {
            return Err(String::from("ピクセル情報取得失敗"));
        }
        Ok(Self {bpp: bmp.header.bpp, pixels})
    }
}

///
/// モノアイコン用パターン構造体
///
#[derive(Default)]
struct BitsPatterns {
    /// 全灯色パターン
    lights: [u16; 16],
    /// 中間色パターン
    harf: [u16; 16],
}

impl BitsPatterns {
    ///
    /// モノアイコン用パターンの作成
    ///
    /// [bppとピクセルのベクタ情報](struct.MyBmpDatas.html)を元に、
    /// [全灯色と中間色のパターン配列](struct.BitsPatterns.html)を返す
    ///
    /// 全灯色判定（パレット番号1 or 白 を全灯色bitと判定）
    /// -     8 bpp ... 0x1 (パレット番号)
    /// -    16 bpp ... 0x8000 (32767)
    /// - 24/32 bpp ... 0xFF FFFF (16777215)
    ///
    fn make (bpp_and_pixels: MyBmpDatas) -> Self {
        let bpp = bpp_and_pixels.bpp;
        let mut pixels = bpp_and_pixels.pixels;
        let mut ptns: Self = Default::default();
        // 結果bitパターン配列作成
        // パレット番号0 or 黒 はスキップ、全灯色以外は中間色と判定
        let lights_color = if bpp == 16 {0x8000} else if bpp >= 24 {0xFF_FFFF} else {1};
        while pixels.len() > 0 {
            // １ピクセル取り出し：座標的には右下→左上に向かう
            let pixel = pixels.pop().unwrap();
            if pixel.color == 0 {
                continue;
            }
            let bitlfg: u16 = 1 << (15 - pixel.x);
            if pixel.color == lights_color {
                ptns.lights[pixel.y as usize] |= bitlfg;
            } else {
                ptns.harf[pixel.y as usize] |= bitlfg;
            }
        }
        ptns
    }

    ///
    /// パターン配列の結果を表示する
    ///
    fn disp (self: Self) {
        println!("\n---モノアイコンパターン ここから---------");
        for ptn in &self.lights {
            print!(" {}", format!("{:04X}", ptn));
        }
        println!("");
        for ptn in &self.harf {
            print!(" {}", format!("{:04X}", ptn));
        }
        println!("\n---モノアイコンパターン ここまで---------");
    }
}

///
/// # BMPファイルを読み込んで CatShanty2 のモノアイコン用bitパターンを表示する #
///
/// コマンドライン引数で BMP ファイルパスを受け取る
///
fn main() {
    const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("");
        // BMP読み込み
        let bpp_and_pixels = MyBmpDatas::load(&args[1]);
        if bpp_and_pixels.is_err() {
            println!("読み込みエラー：{}", bpp_and_pixels.err().unwrap());
            return ();
        }
        // パターン表示
        BitsPatterns::make(bpp_and_pixels.unwrap()).disp();
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
    use super::MyBmpDatas;
    use super::BitsPatterns;

    #[test]
    fn test_make_pattern() {
        let bpp_and_pixels = MyBmpDatas::load("./tests/ren_clip.bmp");
        let patterns = BitsPatterns::make(bpp_and_pixels.unwrap());
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
