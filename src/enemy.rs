use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::attack::Health;

#[derive(Component, Default, Clone)]
pub struct Enemy;

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
	enemy: Enemy,
}

#[derive(Resource, Deref, DerefMut)]
struct EnemyTypes(Vec<EnemyBundle>);

#[derive(Resource, Deref, DerefMut)]
struct Wave(usize);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_startup_system(EnemyPlugin::setup)
			
			.insert_resource(Wave(1))
			.insert_resource(EnemyTypes(Vec::new()))
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
		enemy: Enemy,
        ..Default::default()
    });
		
		
		commands.spawn(enemy_types[0].clone());
	}

	fn spawn_wave(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		enemy_types: Res<EnemyTypes>,
	) {
	}
}
