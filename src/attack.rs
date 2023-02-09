use bevy::prelude::*;
use bevy_rapier2d::prelude::{RapierContext, QueryFilter, Group, CollisionGroups};
use crate::{enemy::Enemy};//, PLAYER_GROUP, ENEMY_GROUP};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
    	app
			.add_system(AttackPlugin::take_damage)
			.add_system(Die::kill_entity)
		;
    }
}


#[derive(Component)]
pub struct Health {
	pub health: usize,
	pub max_health: usize,	
}

#[derive(Component)]
pub struct Die;

struct DeathSpriteAnimation(Handle<Image>);


impl Die {
	fn kill_entity(
		mut commands: Commands,
		mut query: Query<(&Transform, Entity), With<Die>>,
		asset_server: Res<AssetServer>,
	) {
		for (transform, entity) in query.iter_mut() {
			println!("Killed entity: {:?}", entity);
			// commands.spawn_bundle(SpriteSheetBundle {
			//     sprite: todo!(),
			//     texture_atlas: todo!(),
			//     transform: todo!(),
			//     global_transform: todo!(),
			//     visibility: todo!(),
			//     computed_visibility: todo!(),
			// });
			commands.entity(entity).remove::<Die>();
			commands.entity(entity).despawn_recursive();
			
		}
	}
}

struct DeathAnimation;



// Ha ett system så när man sätter en attack på sin spelare som en komponent så attackerar attacken från sig själv
// Ha en strukt metod man kallar med rätt argument för att utföra attackent



//Component put on players or enemies that have been struck by an attack to make them take damage
#[derive(Component)]
pub struct TakeDamage(usize); //Maybe make sparse


impl AttackPlugin {
	fn take_damage(
		mut commands: Commands,
		mut damage_query: Query<(&mut Health, &TakeDamage, Entity), Without<Die>>,
	) {
		for (mut health, damage_taken, entity) in damage_query.iter_mut() {
			commands.entity(entity).remove::<TakeDamage>();
			dbg!("Taking damage.. or am i?");
			if damage_taken.0 > health.health {
				commands.entity(entity).insert(Die);
			}
			else {
				health.health -= damage_taken.0;
			}
		}
		
	}	
}



pub struct Attack;

impl Attack {
	// Style guide name: shoot_
	// 

	pub fn shot_laser(
		commands: &mut Commands,
		// asset_server: Res<AssetServer>,
		rapier_context: &Res<RapierContext>,
		_entity: Entity,
		starting_point: Vec2,
		direction: Vec2,
		// toi: f32,
		// group: Group,
		
	) {
		let toi = f32::MAX;
		if let Some((entity, toi)) = rapier_context.cast_ray(
			Vec2::new(starting_point.x, starting_point.y + 10.0), Vec2::new(0.0, 1.0), toi, false, QueryFilter::new()
		) {
			let hit_point = starting_point + direction * toi;
			println!("Hit entity! {:?} at point {:?}", entity, hit_point);
			if let Some(mut ec) = commands.get_entity(entity) {
			    ec.insert(TakeDamage(20));
			}
			
		}
		else {
			dbg!["Something went wrong.."];
		}
		
		
	}
	
	
}
