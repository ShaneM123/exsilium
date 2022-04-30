use crate::{
	 Player, PlayerState, Speed, SpriteInfos, WinSize,
	 SCALE,  Action, TIME_STEP, PLAYER_SPRITE, load_image,
};
use bevy::{core::FixedTimestep, prelude::*};
use leafwing_input_manager::{prelude::{InputMap, ActionState}, InputManagerBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
        app
        .insert_resource(PlayerState::default())
        .add_system(move_player.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(player_spawn.system()),
        );
}
	}

fn player_spawn(
	mut commands: Commands,
	sprite_infos: Res<SpriteInfos>,
	win_size: Res<WinSize>,
	time: Res<Time>,
	mut player_state: ResMut<PlayerState>,
) {
    let mut input_map = InputMap::default();
    input_map.insert(Action::Left, KeyCode::Left);
    input_map.insert(Action::Right, KeyCode::Right);
    input_map.insert(Action::Up, KeyCode::Up);
    input_map.insert(Action::Down, KeyCode::Down);

	let _now = time.seconds_since_startup();

		let bottom = -win_size.h / 2.;
		commands
			.spawn_bundle(SpriteBundle {
				texture: sprite_infos.player.0.clone(),
				transform: Transform {
					translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
					scale: Vec3::new(SCALE, SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Player)
            .insert_bundle(InputManagerBundle::<Action>{
                input_map,
                ..Default::default()
            })
			.insert(Speed::default());

		player_state.spawned();
	
}

fn move_player(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    //query.for_each(|x| println!("{:?}", x));
    //println!("QUERY: {:?}");
    let (action_state, mut transform) = query.iter_mut().next().unwrap();

    // To only perform the action once when the button is first clicked,
    // use `.just_pressed` instead.
    // To trigger when the click is released, use `.just_released`
    if action_state.pressed(Action::Left) {
        transform.translation.x -= 1.;
    }

    if action_state.pressed(Action::Right) {
        transform.translation.x += 1.;
    }
    if action_state.pressed(Action::Up) {
        transform.translation.y += 1.;
    }

    if action_state.pressed(Action::Down) {
        transform.translation.y -= 1.;
    }
}