use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;
use bevy_rapier2d::prelude::*;

use crate::AppState;
use crate::attack::Alive;
use crate::enemy_util::*;



// Difficulty scale 1 - 5 (5 hardest)
// Amount of enemies are inverted their difficulty scale
// diff 1 = amount 5
// diff 5 = amount 1 (maybe boss or that might be 6)


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_startup_system(EnemyPlugin::setup)
			.add_systems((
				EnemyPlugin::spawn_wave,
				EnemyPlugin::control_enemy_movement,
				EnemyPlugin::end_game,
			).in_set(OnUpdate(AppState::InGame)))
			.add_system(EnemyPlugin::clean_on_exit_in_game.in_schedule(OnExit(AppState::InGame)))
			.insert_resource(Wave(0))
			.insert_resource(WaveTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
			.insert_resource(EnemyTypes(HashMap::new()))
			.insert_resource(MoveEnemyBy(30.0))
		
		;
    }

}

impl EnemyPlugin {
	// Adds the different enemy types into a hashmap to easily spawn them with no boilerplate.
	fn setup(
		asset_server: Res<AssetServer>,
		mut enemy_types: ResMut<EnemyTypes>,
	) {
		
    let mutant_space_mother_texture = asset_server.load("Mutant_SpaceMorphWasp_Mother_B_281x299.png");
    let overlord_nightmare_texture = asset_server.load("Overlord_Nightmare_B_261x235.png");
    let coredefender_nightmare_scarlet = asset_server.load("CoreDefender_F_NightMare_Scarlet_209x182.png");

	enemy_types.insert(EnemyType::MutantSpaceMother, EnemyBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        texture: mutant_space_mother_texture,
	    ridgidbody: RigidBody::Dynamic,
		lockedaxes: LockedAxes::ROTATION_LOCKED,
		collider: Collider::ball(50.0),
		enemy_diff: EnemyDifficulty(1),
		sprite_width: SpriteWidth(281.0),
		enemy_speed: EnemySpeed(50.0),
		velocity: Velocity::zero(), // To make move_enemy_forward query work.
		enemy: Enemy,
        ..Default::default()
    });
		
	enemy_types.insert(EnemyType::OverlordNightmare, EnemyBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        texture: overlord_nightmare_texture,
	    ridgidbody: RigidBody::Dynamic,
		lockedaxes: LockedAxes::ROTATION_LOCKED,
		collider: Collider::ball(50.0),
		enemy_diff: EnemyDifficulty(1),
		sprite_width: SpriteWidth(281.0),
		enemy_speed: EnemySpeed(80.0),
		velocity: Velocity::zero(),
		enemy: Enemy,
        ..Default::default()
    });
		
	enemy_types.insert(EnemyType::CoreDefenderScarlet, EnemyBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        texture: coredefender_nightmare_scarlet,
	    ridgidbody: RigidBody::Dynamic,
		lockedaxes: LockedAxes::ROTATION_LOCKED,
		collider: Collider::ball(50.0),
		enemy_diff: EnemyDifficulty(1),
		sprite_width: SpriteWidth(281.0),
		enemy_speed: EnemySpeed(120.0),
		velocity: Velocity::zero(),
		enemy: Enemy,
        ..Default::default()
    });
		// This is how one would spawn the different ones.
		// commands.spawn(enemy_types.0.get(&EnemyType::MutantSpaceMother).unwrap().clone());
		// commands.spawn(enemy_types.0.get(&EnemyType::OverlordNightmare).unwrap().clone());
		// commands.spawn(enemy_types.0.get(&EnemyType::CoreDefenderScarlet).unwrap().clone());
	}
	// method name is self explanatory.
	fn clean_on_exit_in_game(
        mut commands: Commands,
        query: Query<Entity, With<Enemy>>,
		mut wave: ResMut<Wave>,
			
	) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
	}
	

	fn spawn_wave(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		time: Res<Time>,
		mut wave_timer: ResMut<WaveTimer>,
		enemy_types: Res<EnemyTypes>,
		mut enemy_query: Query<&mut Transform, With<Enemy>>,
		move_enemy_by: Res<MoveEnemyBy>,
		window_query: Query<&Window, With<PrimaryWindow>>,
		mut wave: ResMut<Wave>,
	) {
		
		// If true: The enemies on the screen will have to jump forward so the newly spawned entities have to come into the screen.
		if wave_timer.just_finished() {
			wave.0 += 1;
			let next_enemy_index: EnemyType = rand::random();		
			// eprintln!("{:?}", next_enemy_index);
			let mut next_enemy = enemy_types.0.get(&next_enemy_index).unwrap().clone();
			// Dont worry about it.
			let Ok(window) = window_query.get_single() else {
		        return;
		    };
			let window_width = window.width();
			let most_left_side = (window_width / 2.0) * -1.0;
			let amount_of_enemies_to_spawn = (next_enemy.enemy_diff.0 as isize -5_isize).abs() as usize;
			dbg!("{}", most_left_side);
			
			for amount_of_enemies in 0..amount_of_enemies_to_spawn {
				next_enemy.transform.translation.x = most_left_side + (window_width / amount_of_enemies_to_spawn as f32) * amount_of_enemies as f32 + 50.0;
				commands.spawn(next_enemy.clone());
			}
			
			
			wave_timer.reset();
		}
		else {
			wave_timer.tick(time.delta());
		}
	}

	// When enemies or players go off screen (and possibily touch the player, end the game.)
	fn end_game(
		mut commands: Commands,
		query: Query<(&Transform, Entity), With<Alive>>,
		window_query: Query<&Window, With<PrimaryWindow>>,
		mut next_state: ResMut<NextState<AppState>>,
		
	) {
		let Ok(window) = window_query.get_single() else {
	        return;
	    };
		for (transform, entity) in query.iter() {
			
			// Checks if enemies or players are within the y-axis bounds of the game.
			if transform.translation.y < (window.height() / 2.0) * -1.0 ||
			transform.translation.y > (window.height() / 2.0) {
				eprintln!("delted entity at {}", transform.translation.y);
				commands.entity(entity).despawn_recursive();
				next_state.set(AppState::GameOver);
			}

			// // Checks if enemies or players are within the x-axis bounds of the screen.
			if transform.translation.x > (window.width() / 2.0) ||
			transform.translation.x < (window.width() / 2.0) * -1.0 {
				commands.entity(entity).despawn_recursive();
				next_state.set(AppState::GameOver);
					
			}
		}
		
		
	}
	
	// Moves enemies forward.
	fn control_enemy_movement(
		mut query: Query<&mut Velocity, With<Enemy>>,
		move_enemy_by: Res<MoveEnemyBy>,
	) {
		for mut velocity in query.iter_mut() {
			velocity.linvel.y = **move_enemy_by * -1.0;
			velocity.linvel.x = 0.0;
			
		}
	}
	
}
