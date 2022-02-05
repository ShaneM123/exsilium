use bevy::{
    core::FixedTimestep, ecs::schedule::SystemSet, prelude::*, render::camera::CameraPlugin,
};
use crate::GameState;


struct CellHeight {
    height: f32,
}


struct Cell {
    term: String,
}


#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    row: usize,
    col: usize,
    handle: Handle<Scene>,
}

#[derive(Default)]
pub struct Game {
    player: Player,
    board: Vec<Vec<CellHeight>>,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

const BOARD_SIZE_COLS: usize = 6;
const BOARD_SIZE_ROWS: usize = 6;

// 3.0, 2.0, 0.0 coordinates get desired result
const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_COLS as f32 / 2.2,
    0.0,
    0.0,// BOARD_SIZE_ROWS as f32 - 6.0,
];

pub fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.8;
    camera.transform =
        Transform::from_xyz(2.7, 3.0, 0.0).looking_at(Vec3::from(RESET_FOCUS), Vec3::Y);
    commands.spawn_bundle(camera);
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    // reset the game state

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // spawn the game board
    let cell_scene = asset_server.load("models/AlienCake/tile.glb#Scene0");
    game.board = (0..BOARD_SIZE_ROWS)
        .map(|j| {
            (0..BOARD_SIZE_COLS)
                .map(|i| {
                    let height = 0.5;
                    commands
                        .spawn_bundle((
                            Transform::from_xyz(i as f32, height - 0.2, j as f32),
                            GlobalTransform::identity(),
                        ))
                        .with_children(|cell| {
                            cell.spawn_scene(cell_scene.clone());
                        });
                    CellHeight { height }
                })
                .collect()
        })
        .collect();

        game.player.row = 0;
        game.player.col = BOARD_SIZE_COLS / 2;
    
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(4.0, 5.0, 4.0),
            ..Default::default()
        });
    
    
        // spawn the cruncher character
        game.player.entity = Some(
            commands
                .spawn_bundle((
                    Transform {
                        translation: Vec3::new(game.player.row as f32, 0.0, game.player.col as f32),
                        rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                        ..Default::default()
                    },
                    GlobalTransform::identity(),
                ))
                .with_children(|cell| {
                    cell.spawn_scene(asset_server.load("models/exsilium/bob.glb#Scene0"));
                })
                .id(),
        );


    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Not sure yet:",
            TextStyle {
                font: asset_server.load("fonts/VT323/VT323-Regular.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

// remove all entities that are not a camera
pub fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// change the focus of the camera
pub fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: QuerySet<(
        QueryState<(&mut Transform, &Camera)>,
        QueryState<&Transform>,
    )>,
) {
    const SPEED: f32 = 2.0;
    {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        game.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for (mut transform, camera) in transforms.q0().iter_mut() {
        if camera.name == Some(CameraPlugin::CAMERA_3D.to_string()) {
            *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
        }
    }
}


// restart the game when pressing spacebar
pub fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}




// control the game character
pub fn move_player(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut moved = false;
    let mut rotation = 0.0;
    if keyboard_input.just_pressed(KeyCode::Up) {
        if game.player.row < BOARD_SIZE_ROWS - 1 {
            game.player.row += 1;
        }
        rotation = -std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        if game.player.row > 0 {
            game.player.row -= 1;
        }
        rotation = std::f32::consts::FRAC_PI_2;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        if game.player.col < BOARD_SIZE_COLS - 1 {
            game.player.col += 1;
        }
        rotation = std::f32::consts::PI;
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        if game.player.col > 0 {
            game.player.col -= 1;
        }
        rotation = 0.0;
        moved = true;
    }

    // move on the board
    if moved {
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(game.player.row as f32, 0.0, game.player.col as f32),
            rotation: Quat::from_rotation_y(rotation),
            ..Default::default()
        };
    }

}