mod window;

use window::display_image;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, Texture, TextureDimension, TextureFormat};
use bevy::render::texture::Image;

// #[derive(Resource)]
// struct ImageData {
//     image: image::RgbaImage,
// }

#[derive(Resource)]
struct Display(Handle<Image>);

fn main() {
    // let image_path = "wrench.png".to_owned();
    //
    // let image = image::open(&image_path)
    //     .expect("No image found at provided path")
    //     .to_rgba8();

    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(ImageData { image })
        .add_startup_system(setup)
        //.add_system(draw_image_system)
        .add_system(edit_image)
        .run();

    //display_image("", &image, 500, 500);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_image = asset_server.load("wrench.png");
    commands.insert_resource(Display(handle_image.clone()));
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: handle_image,
        ..default()
    });
}

fn edit_image(handle_image: Res<Display>, mut image_assets: ResMut<Assets<Image>>) {
    if let Some(mut image) = image_assets.get_mut(&handle_image.0) {
        let pixels = &mut image.data;
        for (i, pixel) in pixels.iter_mut().enumerate() {
            match i % 4 {
                0 => *pixel = 100,
                1 => *pixel = 0,
                2 => *pixel = 100,
                3 => *pixel = 255, // alpha
                _ => *pixel = 0,
            }
        }
    }
}

// fn draw_image_system(
//     mut image_data: ResMut<ImageData>,
//     mut texture: ResMut<Texture>,
//     asset_server: Res<AssetServer>,
// ) {
// }
