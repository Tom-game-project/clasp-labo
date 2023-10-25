extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate image; // image ライブラリをインポート
use image::{ImageBuffer, Rgba};

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

    // 画像データをバイト形式で取得
    let png_data = image_buffer.to_vec(); // Vec<u8>
    return png_data;
}


#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
