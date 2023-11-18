use blurhash::encode;
use image::imageops::FilterType::Gaussian;
use image::{DynamicImage, GenericImageView};
use std::env::args;
use std::process::Command;

fn ffmpeg_transcode(input_path: &str) -> Result<DynamicImage, String> {
    let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();
    if ffmpeg_check.is_err() {
        return Err(String::from("ffmpeg not found in PATH"));
    }

    let output = Command::new("ffmpeg")
        .args([
            "-i",
            input_path,
            "-vf",
            "scale=100:-1",
            "-y",
            "-f",
            "image2pipe",
            "-vcodec",
            "png",
            "-",
        ])
        .output()
        .expect("failed to transcode image to png");

    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap());
    }
    Ok(image::load_from_memory(&output.stdout).unwrap())
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("Usage: blurhash <path-to-image>");
        return Err(1);
    }

    let img = match image::open(&args[1]) {
        Ok(img) => img.resize(100, 100, Gaussian),
        Err(_) => ffmpeg_transcode(&args[1]).expect(
            "failed to open image, make sure you have ffmpeg installed and the image is supported",
        ),
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
