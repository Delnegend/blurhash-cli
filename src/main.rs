use blurhash::encode;
use image::imageops::FilterType::Gaussian;
use image::GenericImageView;
use std::env::args;

fn main() -> Result<(), i32> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Usage: blurhash <path-to-image>");
        return Err(1);
    }

    let img_result = image::open(&args[1]);
    let img = match img_result {
        Ok(img) => img.resize(100, 100, Gaussian),
        Err(e) => {
            println!("Error: {}", e);
            return Err(1);
        }
    };

    let (width, height) = img.dimensions();
    let (mut x, mut y) = (4, 3);
    if width < height {
        y = (x as f32 * height as f32 / width as f32).round() as u32;
    } else {
        x = (y as f32 * width as f32 / height as f32).round() as u32;
    }
    x = x.min(9);
    y = y.min(9);

    println!("{}x{}", x, y);

    let blurhash = encode(x, y, width, height, &img.to_rgba8().into_vec());
    println!("{}", blurhash);
    return Ok(());
}
