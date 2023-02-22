fn main() {
    use imageproc::window::display_image;
    use std::env;
    println!("{}", env::var("PWD").unwrap());

    let image_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No image path provided. Using default image.");
            "wrench.png".to_owned()
        }
    };

    let image = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgba8();

    display_image("", &image, 500, 500);
}