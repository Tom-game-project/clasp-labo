extern crate wasm_bindgen;
use wasm_bindgen::{prelude::*};

extern crate image; // image ライブラリをインポート
use image::{ DynamicImage,ImageBuffer, Rgba};

use std::io::Cursor;

#[wasm_bindgen]
pub fn test_image_gen(width:u32,height:u32)->Vec<u8>{

    // Rgba ピクセル値を持つ ImageBuffer を作成
    let mut image_buffer = ImageBuffer::new(width, height);

    // 画像を走査し、ピクセル値を設定
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        // 例: 赤い水平線
        if y == height / 2 {
            *pixel = Rgba([255, 0, 0, 255]);
        }
    }

    let dynamic_image: DynamicImage = DynamicImage::ImageRgba8(image_buffer).into();

    // バッファをメモリに書き込むためのカーソルを用意
    let mut buffer = Cursor::new(Vec::new());

    // DynamicImage を PNG 形式でエンコードし、バッファに書き込む
    dynamic_image.write_to(&mut buffer, image::ImageOutputFormat::Png).unwrap();

    // バッファの内容を Vec<u8> として取得
    let png_data = buffer.into_inner();

    png_data
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        println!("{}","");
    }
}
