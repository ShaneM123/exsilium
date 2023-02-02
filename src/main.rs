use bevy::{prelude::*, sprite::MaterialMesh2dBundle, text::Text2dBounds};
mod menu;
use crate::menu::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_startup_system(initial_setup)
        // Declare the game state, and set its startup value
        .add_state(GameState::Splash)
        // Adds the plugins for each state
        .add_plugin(menu::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

    fn initial_setup(
        mut commands: Commands,
    ) {
        commands.spawn(Camera2dBundle::default());
    }


mod game {
    use bevy::{prelude::*, text::Text2dBounds, sprite::MaterialMesh2dBundle};

    use crate::menu::{self, OnGameScreen};

    use super::{ DisplayQuality, GameState, Volume};

    // This plugin will contain the game. In this case, it's just be a screen that will
    // display the current settings for 5 seconds before returning to the menu
    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
                .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
                .add_system_set(
                    SystemSet::on_exit(GameState::Game).with_system(menu::despawn_screen::<OnGameScreen>),
                );
        }
    }


    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    fn game_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        //let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            font,
        };
        let box_size = Vec2::new(300.0, 200.0);
        let box_position = Vec2::new(0.0, 250.0);
        commands.spawn(Text2dBundle {
            text: Text::from_section("Conscious City", text_style),
            text_2d_bounds: Text2dBounds {
                // Wrap text in the rectangle
                size: box_size,
            },
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            ..default()
        });
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        });

        // Spawn a 5 seconds timer to trigger going back to the menu
        commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    }


    // Tick the timer, and change state when finished
    fn game(
        time: Res<Time>,
        mut game_state: ResMut<State<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu).unwrap();
        }
    }
}