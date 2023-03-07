use bevy::prelude::*;
use std::{
	collections::HashMap,
	convert::From,
};
use bevy_rapier2d::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 

use crate::{attack::Health, player::Player};


impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        match rng.gen_range(0..=7) { // rand 0.8
            a => EnemyType::from(a as usize),
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct LastSpriteSize(pub f32, pub f32);

#[derive(Component, Default, Clone)]
pub struct EnemyDifficulty(pub usize);

#[derive(Component, Default, Clone)]
pub struct SpriteWidth(pub f32);

#[derive(Component, Default, Clone)]
pub struct EnemySpeed(pub f32);

#[derive(Resource, Default, Clone, Deref, DerefMut)]
pub struct MoveEnemyBy(pub f32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EnemyType {
	MutantSpaceMother,
	BuggyBlue,
	BuggyRed,
	BuggyGreen,
	OverlordNightmare,
	CoreDefenderScarlet,
	CoreDefenderScarletDarkness,
	CoreDefenderJudement,
}


impl From<usize> for EnemyType {
	fn from(value: usize) -> EnemyType {
		match value {
			// 0 => EnemyType::BuggyBlue,
			// 1 => EnemyType::BuggyRed,
			// 2 => EnemyType::BuggyGreen,
			3 => EnemyType::OverlordNightmare,
			4 => EnemyType::CoreDefenderScarlet,
			// 5 => EnemyType::CoreDefenderScarletDarkness,
			// 6 => EnemyType::CoreDefenderJudement,
			_ => EnemyType::MutantSpaceMother,
		}
	}
}

#[derive(Bundle, Default, Clone)]
pub struct EnemyBundle {
	pub sprite: Sprite,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub texture: Handle<Image>,
	pub health: Health,
	pub enemy_speed: EnemySpeed,
	pub ridgidbody: RigidBody,
	pub collider: Collider,
	pub lockedaxes: LockedAxes,
	pub visibility: Visibility,
	pub computed_visisbility: ComputedVisibility,
	pub enemy_diff: EnemyDifficulty,
	pub sprite_width: SpriteWidth,
	pub velocity: Velocity,
	pub enemy: Enemy,
}

#[derive(Resource, Deref, DerefMut)]
pub struct EnemyTypes(pub HashMap<EnemyType, EnemyBundle>);

#[derive(Resource, Deref, DerefMut)]
pub struct Wave(pub usize);

// When next wave gone be.
#[derive(Resource, Deref, DerefMut)]
pub struct WaveTimer(pub Timer);


#[derive(Component, Default, Clone)]
pub struct Enemy;
