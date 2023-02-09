use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::attack::Health;

#[derive(Component)]
pub struct Enemy;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_startup_system(EnemyPlugin::setup)
		;
    }
}

impl EnemyPlugin {
	fn setup(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
	) {
		
    let space_ship_texture = asset_server.load("Mutant_SpaceMorphWasp_Mother_B_281x299.png");

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        texture: space_ship_texture,
        ..Default::default()
    })
	.insert(Health {
	    health: 100,
	    max_health: 100,
	})
    .insert(RigidBody::Dynamic)
	.insert(LockedAxes::ROTATION_LOCKED)
	.insert(Collider::ball(50.0))
	.insert(Enemy);
		
	}
}
