extern crate argon2;
extern crate image;

fn main() {
    println!("Hello, world!");
    let hash = hash(b"sup");
    println!("{:?}", hash);
    let image = generate_image(&hash);
    image.save("test.png").unwrap();
}

fn hash(key: &[u8]) -> Vec<u8> {
    let salt: &[u8] = &[0; 8];
    let mut config = argon2::Config::default();
    config.hash_length = 128;
    let hash = argon2::hash_raw(key, salt, &config).unwrap();
    
    hash
}

fn generate_image(data: &[u8]) -> image::RgbImage {
    let mut img: image::RgbImage = image::ImageBuffer::new(6, 6);
    let mut data_iter = data.iter();
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = data_iter.next().unwrap();
        println!("{:?}", r);
        let g = data_iter.next().unwrap();
        println!("{:?}", g);
        let b = data_iter.next().unwrap();
        println!("{:?}", b);
        *pixel = image::Rgb([*r, *g, *b]);
    }
    println!("{:?}", img);

    img
}

