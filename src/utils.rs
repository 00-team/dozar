use rand::Rng;
use std::path::Path;
use std::io;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use crate::config::Config;

pub fn phone_validator(phone: &str) -> bool {
    if phone.len() != 11 || !phone.starts_with("09") {
        return false;
    }

    if phone.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    true
}

pub fn now() -> i64 {
    chrono::Local::now().timestamp()
}

pub fn get_random_string(charset: &[u8], len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| charset[rng.gen_range(0..charset.len())] as char).collect()
}

pub fn get_random_bytes(len: usize) -> String {
    let mut rng = rand::thread_rng();
    hex::encode((0..len).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>())
}

pub fn save_photo(path: &Path, name: &str) -> io::Result<()> {
    let img = ImageReader::open(path)?
        .with_guessed_format()?
        .decode()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    img.thumbnail(512, 512)
        .save_with_format(
            Path::new(Config::RECORD_DIR).join(name),
            ImageFormat::Png,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}
