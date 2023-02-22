mod window;

use window::display_image;

use bevy::prelude::*;
use bevy::render::camera::RenderTarget::Image;
use bevy::render::render_resource::Texture;

// #[derive(Resource)]
// struct ImageData {
//     image: image::RgbaImage,
// }

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
        .run();

    //display_image("", &image, 500, 500);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("wrench.png"),
        ..default()
    });
}

// fn draw_image_system(
//     mut image_data: ResMut<ImageData>,
//     mut texture: ResMut<Texture>,
//     asset_server: Res<AssetServer>,
// ) {
// }
