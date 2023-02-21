use bevy::prelude::*;
use std::collections::HashMap;
use bevy_rapier2d::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 

use crate::{attack::Health, player::Player};


#[derive(Component, Default, Clone)]
pub struct EnemyDifficulty(pub usize);

#[derive(Component, Default, Clone)]
pub struct SpriteWidth(pub f32);

#[derive(Component, Default, Clone)]
pub struct EnemySpeed(pub f32);

#[derive(Resource, Default, Clone, Deref, DerefMut)]
pub struct MoveEnemyBy(pub f32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        todo!()
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
