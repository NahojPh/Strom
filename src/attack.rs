use bevy::prelude::*;

pub struct AttackPlugin;


#[derive(Component)]
pub struct Health {
	pub health: usize,
	pub max_health: usize,	
}

#[derive(Component)]
pub struct Die;

#[derive(Component)]
pub struct TakeDamage(usize);

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
    	app
			.add_system(AttackPlugin::take_damage)
		;
    }
}



impl AttackPlugin {
	fn take_damage(
		mut commands: Commands,
		damage_query: Query<(&mut Health, &TakeDamage, Entity)>,
	) {
		for (mut health, damage_taken, entity) in damage_query.iter_mut() {
			commands.entity(entity).remove::<TakeDamage>();
			if damage_taken.0 > health.health {
				commands.entity(entity).insert(Die);
			}
			else {
				health.health -= damage_taken.0;
			}
		}
		
	}	
}



