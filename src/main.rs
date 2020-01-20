extern crate image;

use std::env;

use image::Pixel;
use image::imageops;
use image::GenericImageView;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);

    let img = image::open(&args[1]).unwrap();
    let (ox, oy) = img.dimensions();
    let x = args[2].parse().unwrap();
    let y = (oy * x * 10)/ (ox * 16);

    let img = img.resize_exact(x, y, imageops::FilterType::Nearest);

    let s = String::from("MND8OZ$7I?+=~:,..");

    for (x, y, p) in img.pixels() {
        if x == 0 && y != 0 {
            println!("");
        }
        let image::Luma(data) = p.to_luma();
        let i = data[0] as usize *16/255;
        print!("{}", s.chars().nth(i).unwrap());
    }
}