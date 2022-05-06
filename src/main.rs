// #![allow(unused)] // silence unused warnings while learning
use std::path::Path;

use player::PlayerPlugin;

use bevy::{prelude::*, render::{texture::{self, ImageType, CompressedImageFormats}, render_resource::Texture}};
use leafwing_input_manager::prelude::*;

mod player;

const PLAYER_SPRITE: &str = "ship_B.png";
const SPRITE_DIR: &str = "assets/models/exsilium/";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const PLAYER_RESPAWN_DELAY: f64 = 2.;

// region:    Resources
pub struct SpriteInfos {
	player: (Handle<Image>, Vec2),
}
struct WinSize {
	#[allow(unused)]
	w: f32,
	h: f32,
}

#[derive(Component, Debug)]
struct Player;


#[derive(Component)]
struct FromPlayer;

struct PlayerState {
	on: bool,
}
impl Default for PlayerState {
	fn default() -> Self {
		Self {
			on: true,
		}
	}
}
impl PlayerState {
	fn spawned(&mut self) {
		self.on = true;
	}
}

#[derive(Component)]
struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.)
	}
}


#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Left,
    Right,
    Up,
    Down,
}

fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {

    let path = Path::new(SPRITE_DIR).join(path);
    let bytes = std::fs::read(&path).expect(&format!("cannot find {}", path.display()));
    let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png"), CompressedImageFormats::all(), false).unwrap();
    let size = image.texture_descriptor.size;
    let size = Vec2::new(size.width as f32, size.height as f32);
    let image_handle = images.add(image);
    (image_handle, size)
}


fn setup(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
    mut images: ResMut<Assets<Image>>,
) {
	let window = windows.get_primary_mut().unwrap();

	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

	commands.insert_resource(SpriteInfos {
		player: load_image(&mut images, PLAYER_SPRITE),
	});

	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});

	// position window
	// Commented out - when recording tutorial (place as you see fit)
	// window.set_position(IVec2::new(3870, 4830));
}

fn spawn_ui(mut commands: Commands, mut player_query: Query<Entity, With<Player>>) {
    let player_entity = player_query.single();
    // Left
    let left_button = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(150.0)),
                ..Default::default()
            },
            color: Color::RED.into(),
            ..Default::default()
        })
        // This component links the button to the entity with the `ActionState` component
        .insert(ActionStateDriver {
            action: Action::Left,
            entity: player_entity,
        })
        .id();

    // Right
    let right_button = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(150.0)),
                ..Default::default()
            },
            color: Color::BLUE.into(),
            ..Default::default()
        })
        .insert(ActionStateDriver {
            action: Action::Right,
            entity: player_entity,
        })
        .id();

    // Container for layout
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .push_children(&[left_button, right_button]);
}



fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "--Exsilium--".to_string(),
			width: 598.0,
			height: 676.0,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
         .add_plugin(InputManagerPlugin::<Action>::default())
		 .add_startup_system(setup)
         .add_plugin(PlayerPlugin)
         .add_system(spawn_ui.after("game_setup_actors"))
		.run();
}