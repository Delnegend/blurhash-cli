use blurhash::encode;
use image::imageops::FilterType::Gaussian;
use image::GenericImageView;
use std::env::args;
use std::fs;

mod transcode;

fn is_directory(path: &str) -> bool {
    let metadata = fs::metadata(path).unwrap();
    metadata.is_dir()
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 || is_directory(&args[1]) {
        eprintln!("Usage: blurhash <path-to-image>");
        return Err(1);
    }

    let img = match transcode::transcode(&args[1]) {
        Ok(img) => img.resize(100, 100, Gaussian),
        Err(_) => return Err(1),
    };

    let (width, height) = img.dimensions();
    let (x, y) = {
        if width < height {
            (3, (3.0 * height as f32 / width as f32).round() as u32)
        } else {
            ((3.0 * width as f32 / height as f32).round() as u32, 3)
        }
    };

    let blurhash = encode(x, y, width, height, &img.to_rgba8().into_vec());
    println!("{} {} {}", blurhash.unwrap(), x, y);
    Ok(())
}
