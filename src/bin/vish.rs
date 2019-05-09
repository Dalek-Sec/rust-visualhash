use std::env;

use visualhash::*;

fn main() {
    let key = env::args()
        .last()
        .expect("Missing argument to use as data for image");
    println!("Visualizing '{}'", &key);
    let key: &[u8] = key.as_bytes();
    let img = visualize(key).unwrap();
    img.save("visual.png").unwrap();
    println!("Saved as visual.png")
}
