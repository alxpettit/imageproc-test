mod window;

use window::display_image;

fn main() {
    let image_path = "wrench.png".to_owned();

    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgba8();

    display_image("", &image, 500, 500);
}
