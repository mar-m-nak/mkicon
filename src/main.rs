
// #[macro_use]
// extern crate load_file;

// use std::io;
use std::env;

// use tinybmp::{Bmp, FileType, Header, Pixel};
use tinybmp::{Bmp, Pixel};
use load_file::load_bytes;

// https://www.setsuki.com/hsp/ext/bmp.htm
// https://docs.rs/tinybmp/0.2.3/tinybmp/
// https://webbibouroku.com/Blog/Article/rust-iter-index


// struct Bits {
//     b0: u8,
//     b1: u8,
//     b2: u8,
//     b3: u8,
//     b4: u8,
//     b5: u8,
//     b6: u8,
//     b7: u8,
// }


fn read_bmp(path: &str) {

    // println!("BMP ファイル");
    // let mut path = String::new();

    // io::stdin()
    //     .read_line(&mut path)
    //     .expect("行の読み込みに失敗しました");

    // let ppath = &path.trim();

    // let bmp = Bmp::from_slice(include_bytes!("../tests/test1_256.bmp"))
    let bmp = Bmp::from_slice(load_bytes!(path))
        .expect("Failed to parse BMP image");

    // BMPヘッダーを読み取る
    // assert_eq!(
    //     bmp.header,
    //     Header {
    //         file_type: FileType::BM,
    //         file_size: 314,
    //         reserved_1: 0,
    //         reserved_2: 0,
    //         image_data_start: 122,
    //         bpp: 24,
    //         image_width: 8,
    //         image_height: 8,
    //         image_data_len: 192
    //     }
    // );

    println!("SIZE: w {} px , h {} px", bmp.header.image_width, bmp.header.image_height);
    println!("BPP: {}", bmp.header.bpp);

    // 生の画像データスライスが正しい長さであることを確認します（解析されたヘッダーによる）
    // assert_eq!(bmp.image_data().len(), bmp.header.image_data_len as usize);

    // この画像のピクセル座標と色のイテレータを取得し、vecに収集します
    let pixels: Vec<Pixel> = bmp.into_iter().collect();

    // 読み込まれたサンプル画像は8x8pxです
    // assert_eq!(pixels.len(), 8 * 8);

    println!("pixels length ... {}", pixels.len());

    // for (i, val) in pixels.iter().enumerate() {
    //     println!("{} ... {},{} col: {}", i, val.x, val.y, val.color);
    // }

    // for j in (0..256).step_by(16) {
    //     for i in (0..16).rev() {
    //         let z = j + i;
    //         println!("{} ... {},{} col: {}", z, pixels[z].x, pixels[z].y, pixels[z].color);
    //     }
    // }

    let mut a_lines: [u16; 16] = [0; 16];
    let mut b_lines: [u16; 16] = [0; 16];
    let mut pos: usize = 0;

    for posy in (0..256).step_by(16) {

        let mut a_line: u16 = 0;
        let mut b_line: u16 = 0;

        // 全灯色の判定 1 or 白
        // bpp  8 bit = 1（パレット番号）
        // bpp 16 bit = 32767
        // bpp 24 bit = 16777215
        // bpp 32 bit = 16777215
        let lights_color = if bmp.header.bpp == 16 {32767} else if bmp.header.bpp >= 24 {16777215} else {1};

        for posx in (0..16).rev() {
            pos = posy + posx;
            if pixels[pos].color == 0 {continue;}
            let bitflg:u16 = 1 << (15 - posx);
            if pixels[pos].color == lights_color {
                a_line = a_line | bitflg;
            } else {
                b_line = b_line | bitflg;
            }
            // println!("... {:?}", bitflg);
        }
        a_lines[pos / 16] = a_line;
        b_lines[pos / 16] = b_line;
        // println!("{} --> {}", pos, line);
    }

    for ptn in &a_lines {
        print!(" {}", format!("{:04X}", ptn));
    }
    println!("");
    for ptn in &b_lines {
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
