use std::{fs, process::Command};

use image::DynamicImage;

fn is_binary_in_path(binary: &str) -> bool {
    Command::new(binary).output().is_ok()
}

fn ffmpeg(input_path: &str) -> Result<DynamicImage, String> {
    if !is_binary_in_path("ffmpeg") {
        return Err("ffmpeg not found in path".to_string());
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
        let err = String::from_utf8(output.stderr).unwrap();
        fs::write("blurhash.log", err).expect("failed to write ffmpeg.log");
        return Err(
            "ffmpeg failed to transcode image to png, check the blurhash.log file".to_string(),
        );
    }
    Ok(image::load_from_memory(&output.stdout).unwrap())
}

fn jpegxl(input_path: &str) -> Result<DynamicImage, String> {
    if !is_binary_in_path("djxl") {
        return Err("djxl not found in path".to_string());
    }

    let output_path = format!("{}.png", input_path);

    let output = Command::new("djxl")
        .args([input_path, &output_path])
        .output()
        .expect("failed to transcode image to png");

    if !output.status.success() {
        return Err("djxl failed to transcode image to png".to_string());
    }

    let image_data = image::open(format!("{}.png", input_path)).unwrap();
    fs::remove_file(format!("{}.png", input_path)).unwrap();
    Ok(image_data)
}

pub fn transcode(input_path: &str) -> Result<DynamicImage, String> {
    match input_path.split('.').last().unwrap() {
        "png" | "jpg" | "jpeg" => Ok(image::open(input_path).unwrap()),
        "jxl" => jpegxl(input_path),
        _ => ffmpeg(input_path),
    }
}
