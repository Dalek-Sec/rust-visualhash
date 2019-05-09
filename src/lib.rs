extern crate argon2;
extern crate image;

/// Convert some bytes into a (hopefully) unique looking image  
/// Can easily be saved to a file with `.save()`
pub fn visualize(key: &[u8]) -> image::RgbImage {
    let hash = hash(key);
    let img = generate_image(&hash);

    img
}

/// Use KDF (to increase expense to find keys with valid look-alike images)
/// with consistent parameters (to be deterministic)
fn hash(key: &[u8]) -> Vec<u8> {
    let salt: &[u8] = &[0; 8];
    let mut config = argon2::Config::default();
    config.hash_length = 128;
    let hash = argon2::hash_raw(key, salt, &config).unwrap();

    hash
}

/// For each RGB value for each pixel in the image, use a byte from the hash
fn generate_image(data: &[u8]) -> image::RgbImage {
    let mut img: image::RgbImage = image::ImageBuffer::new(6, 6);
    let mut data_iter = data.iter();
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let r = data_iter.next().unwrap();
        let g = data_iter.next().unwrap();
        let b = data_iter.next().unwrap();
        *pixel = image::Rgb([*r, *g, *b]);
    }

    img
}
