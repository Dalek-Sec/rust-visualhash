extern crate argon2;
extern crate image;

/// Convert some bytes into a (hopefully) unique looking image.
/// The returned image can easily be saved to a file with `.save("filename.format")`.
pub fn visualize(key: &[u8]) -> Result<image::RgbImage, argon2::Error> {
    let hash = hash(key)?;
    let img = generate_image(6, 6, &hash);

    Ok(img)
}

/// Use KDF (to increase expense to find keys with valid look-alike images)
/// with consistent parameters (to be deterministic).
fn hash(key: &[u8]) -> Result<Vec<u8>, argon2::Error> {
    let salt: &[u8] = &[0; 8];
    let config = argon2::Config {
        variant: argon2::Variant::Argon2d,
        version: argon2::Version::Version13,
        mem_cost: 65536,
        time_cost: 12,
        lanes: 4,
        thread_mode: argon2::ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 128
    };
    let hash = argon2::hash_raw(key, salt, &config)?;

    Ok(hash)
}

/// For each RGB value for each pixel in the image, use a byte from the hash.
/// Therefore, the length of the data must exceed the number of pixels.
fn generate_image(height: u32, width: u32, data: &Vec<u8>) -> image::RgbImage {
    assert!(data.len() as u32 >= height*width);
    let mut img: image::RgbImage = image::ImageBuffer::new(width, height);
    let mut data_iter = data.iter();
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r = data_iter.next().unwrap();
        let g = data_iter.next().unwrap();
        let b = data_iter.next().unwrap();
        *pixel = image::Rgb([*r, *g, *b]);
    }

    img
}
