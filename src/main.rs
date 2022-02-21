
use std::path::Path;

use bevy::{prelude::*, render::{texture::{self, ImageType}, render_resource::Texture}};

const PLAYER_SPRITE: &str = "ship_B.png";
const SPRITE_DIR: &str = "assets/models/exsilium/";

fn main(){
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
    .insert_resource(WindowDescriptor{
        title: "Radar v0.1".to_string(),
        width: 800.0,
        height: 600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut windows: ResMut<Windows>,
)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(3870, 4830));

    //spawn sprite
    let bottom = -window.height() / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: load_image(&mut images, PLAYER_SPRITE).0,
        sprite: Sprite{
            custom_size: Some(Vec2::new(200.0,100.0), ),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., bottom + 75.0 / 2.0 + 5.0,10.0),
            ..Default::default()
        },
        ..Default::default()
    });
    ()
}

fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {

    let path = Path::new(SPRITE_DIR).join(path);
    let bytes = std::fs::read(&path).expect(&format!("cannot find {}", path.display()));
    let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
    let size = image.texture_descriptor.size;
    let size = Vec2::new(size.width as f32, size.height as f32);
    let image_handle = images.add(image);
    (image_handle, size)
}