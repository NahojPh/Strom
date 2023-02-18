use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::attack::Health;

#[derive(Component, Default, Clone)]
pub struct Enemy;


// Difficulty scale 1 - 5 (5 hardest)
// Amount of enemies are inverted their difficulty scale
// diff 1 = amount 5
// diff 5 = amount 1 (maybe boss or that might be 6)
#[derive(Component, Default, Clone)]
pub struct EnemyDifficulty(pub usize);

#[derive(Component, Default, Clone)]
pub struct SpriteWidth(pub f32);

#[derive(Resource, Default, Clone, Deref, DerefMut)]
pub struct MoveEnemyBy(pub f32);



#[derive(Bundle, Default, Clone)]
pub struct EnemyBundle {
	sprite: Sprite,
	transform: Transform,
	global_transform: GlobalTransform,
	texture: Handle<Image>,
	health: Health,
	ridgidbody: RigidBody,
	collider: Collider,
	lockedaxes: LockedAxes,
	visibility: Visibility,
	computed_visisbility: ComputedVisibility,
	enemy_diff: EnemyDifficulty,
	sprite_width: SpriteWidth,
	enemy: Enemy,
}

#[derive(Resource, Deref, DerefMut)]
struct EnemyTypes(Vec<EnemyBundle>);

#[derive(Resource, Deref, DerefMut)]
struct Wave(usize);

// When next wave gone be.
#[derive(Resource, Deref, DerefMut)]
struct WaveTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_startup_system(EnemyPlugin::setup)
			.add_system(EnemyPlugin::spawn_wave)
			
			.insert_resource(Wave(1))
			.insert_resource(WaveTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
			.insert_resource(EnemyTypes(Vec::new()))
			.insert_resource(MoveEnemyBy(200.0))
		
		;
    }
}

impl EnemyPlugin {
	fn setup(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		mut enemy_types: ResMut<EnemyTypes>,
	) {
		
    let space_ship_texture = asset_server.load("Mutant_SpaceMorphWasp_Mother_B_281x299.png");

	enemy_types.push(EnemyBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        texture: space_ship_texture,
	    ridgidbody: RigidBody::Dynamic,
		lockedaxes: LockedAxes::ROTATION_LOCKED,
		collider: Collider::ball(50.0),
		enemy_diff: EnemyDifficulty(1),
		sprite_width: SpriteWidth(281.0),
		enemy: Enemy,
        ..Default::default()
    });
		println!("{:?}", enemy_types[0].sprite);
		
		commands.spawn(enemy_types[0].clone());
	}

	fn spawn_wave(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		time: Res<Time>,
		mut wave_timer: ResMut<WaveTimer>,
		enemy_types: Res<EnemyTypes>,
		mut enemy_query: Query<&mut Transform, With<Enemy>>,
		move_enemy_by: Res<MoveEnemyBy>,
		windows: Res<Windows>,
		mut wave: ResMut<Wave>,
	) {
		// If true: The enemies on the screen will have to jump forward so the newly spawned entities have to come into the screen.
		if wave_timer.just_finished() {
			wave.0 += 1;
			let next_enemy_index = rand::thread_rng().gen_range(0..enemy_types.len());		
			for mut transform in enemy_query.iter_mut() {
				transform.translation.y -= move_enemy_by.0 + 10.0;
			}
			let mut next_enemy = enemy_types[next_enemy_index].clone();
			// Dont worry about it.
			let window_width = windows.get_primary().expect("Window is not found. Please send help").width();
			let padding = 50.0;
			let most_left_side = (window_width / 2.0) * -1.0;
			let amount_of_enemies_to_spawn = (next_enemy.enemy_diff.0 as isize -5_isize).abs() as usize;
			dbg!("{}", most_left_side);
			
			let mut enemy_spawn_x_translation = most_left_side;
			
			for amount_of_enemies in 0..amount_of_enemies_to_spawn {
				next_enemy.transform.translation.x = most_left_side + (window_width / amount_of_enemies_to_spawn as f32)*amount_of_enemies as f32 + 50.0;
				commands.spawn(next_enemy.clone());
				enemy_spawn_x_translation += next_enemy.sprite_width.0;
			}
			
			
			wave_timer.reset();
		}
		else {
			wave_timer.tick(time.delta());
		}
		
			
	}
}
