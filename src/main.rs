use bevy::{prelude::*};

mod board;
mod menu;
mod splash;

const TEXT_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

const MAIN_MENU_WIDTH: f32 = 130.00;
const MAIN_MENU_HEIGHT: f32 = 400.00;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
    Playing,
    GameOver,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_startup_system(setup)
        // Declare the game state, and set its startup value
        .add_state(GameState::Splash)
        // Adds the plugins for each state
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

mod game {
    use bevy::{prelude::*, core::FixedTimestep};

    use crate::board::{Game,setup_cameras,setup,teardown,gameover_keyboard, focus_camera, move_player};

    use super::{GameState};

    // This plugin will contain the game(3d board)
    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        
        fn build(&self, app: &mut App) {
            app.insert_resource(Msaa { samples: 4 })
    .init_resource::<Game>()
    .add_startup_system(setup_cameras)
    .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            .with_system(focus_camera)
            .with_system(move_player.system())
    )
    .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
    .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(gameover_keyboard))
    .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(5.0))
    ).add_system(bevy::input::system::exit_on_esc_system);
    
        }

    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}