use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::{RapierContext, QueryFilter, Group, CollisionGroups};
use crate::{sprite_animation::*, AppState};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
    	app
			.add_startup_system(AttackPlugin::setup)
			.add_system(AttackPlugin::animate_laser.in_set(OnUpdate(AppState::InGame)))
			.add_system(AttackPlugin::render_game_over_text.in_schedule(OnEnter(AppState::GameOver)))
			.add_system(AttackPlugin::game_over_transition.in_schedule(OnExit(AppState::GameOver)))
		;
    }
}


#[derive(Component, Clone)]
pub struct Health {
	pub health: usize,
	pub max_health: usize,	
}
impl Default for Health {
    fn default() -> Self {
       Health {
	        health: 100,
	        max_health: 100,
	    }
    }
}

#[derive(Resource)]
pub struct DeathSpriteAnimation {
	texture_atlas_handle: Handle<TextureAtlas>,
    animation_indices: AnimationIndices,
}


#[derive(Component)]
pub struct LaserAnimation {
	timer: Timer,
	starting_point: Vec3,
	hit_point: Vec3,
}

#[derive(Component)]
struct GameOverText(Timer);


impl AttackPlugin {
	fn setup(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	) {
	    let texture_handle = asset_server.load("effects/16_sunburn_spritesheet.png");
	    let texture_atlas =
	        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 8, 8, None, None);
	    let texture_atlas_handle = texture_atlases.add(texture_atlas);
	    // Use only the subset of sprites in the sheet that make up the run animation
	    let animation_indices = AnimationIndices { first: 1, last: 32 };

		commands.insert_resource(DeathSpriteAnimation {
		    texture_atlas_handle,
		    animation_indices,
		});
	}
	
	fn render_game_over_text(
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		window_query: Query<&Window, With<PrimaryWindow>>,
		mut next_state: ResMut<NextState<AppState>>,
	) {
		let Ok(window) = window_query.get_single() else {
	        return;
	    };
	
		commands.spawn((TextBundle::from_section(
			"Game Over Idiot",
			TextStyle {
			    font: asset_server.load("./fonts/Roboto-Black.ttf"),
			    font_size: 150.0,
			    color: Color::WHITE,
			},
		)
		.with_text_alignment(TextAlignment::Center)
		.with_style(Style {
			position_type: PositionType::Absolute,
			position: UiRect {
			
			    right: Val::Px(window.width() / 2.0),
			    bottom: Val::Px(window.height() / 2.0),
				..Default::default()
			},
			..Default::default()	
		}),
		GameOverText(Timer::from_seconds(3.0, TimerMode::Once)),
		));

		next_state.set(AppState::MainMenu);
		

	
	
	}

	fn game_over_transition(
		mut commands: Commands,
		query: Query<Entity, With<GameOverText>>,
	) {
		// This is often bad to do (sleep a thread in bevy) but because nothing should be happening while 
		// the game over screen is happening this is fine.
		std::thread::sleep(Duration::from_secs(2));

		for entity in query.iter() {
			commands.entity(entity).despawn_recursive();
		}
		
	}
	
	
		
	fn animate_laser(
		mut commands: Commands,
		time: Res<Time>,
		mut query: Query<(&mut Transform, &mut LaserAnimation, Entity)>,
	) { // 1 - delen delas på det hela
		// i lerpen
		for (mut transform, mut laser_animation, entity) in query.iter_mut() {
			if laser_animation.timer.finished() {
				commands.entity(entity).despawn_recursive();
			}
			else {
				laser_animation.timer.tick(time.delta());

				transform.translation = laser_animation.starting_point.lerp(Vec3::new(laser_animation.hit_point.y, laser_animation.hit_point.x, laser_animation.hit_point.z), laser_animation.timer.percent());
			}
		}
	
	}
}


pub fn take_damage(
	commands: &mut Commands,
	entity: &mut Entity,
	health: &mut Health,
	damage_taken: usize,
	translation: Vec3,
	death_sprite_animation: &Res<DeathSpriteAnimation>,
) {
	// dbg!("Taking damage.. or am i?");
	if damage_taken > health.health {
		commands.spawn(SpriteSheetBundle {
		    sprite: TextureAtlasSprite::new(death_sprite_animation.animation_indices.first),
		    texture_atlas: death_sprite_animation.texture_atlas_handle.clone(),
		    transform: Transform {
		        translation,
		        scale: Vec3::splat(3.0),
				..Default::default()
		    },
			..Default::default()
		})
		.insert(AnimationIndices::from(death_sprite_animation.animation_indices.clone()))
		.insert(AnimationTimer(Timer::new(Duration::from_millis(5), TimerMode::Repeating)));

		commands.entity(*entity).despawn_recursive();
	}
	else {
		health.health -= damage_taken;
	}
}



pub struct Attack;

impl Attack {
	// Style guide name: shoot_
	// 
// Måste lägga till en "laser" på laser attacken så det sys
//	tänkte 'lerpa' mellan spelaren och fienden.
	
	pub fn shot_laser(
		commands: &mut Commands,
		rapier_context: &Res<RapierContext>,
		starting_point: Vec2,
		direction: Vec2,
		death_sprite_animation: &Res<DeathSpriteAnimation>,
		health: &mut Health,
	) {
		let toi = f32::MAX;
		if let Some((mut entity, real_toi)) = rapier_context.cast_ray(
			Vec2::new(starting_point.x, starting_point.y + 10.0), Vec2::new(0.0, 1.0), toi, false, QueryFilter::new()
		) {
			let hit_point = starting_point + direction * real_toi;
			// println!("Hit entity! {:?} at point {:?}", entity, hit_point);
			if let Some(mut _ec) = commands.get_entity(entity) {
				// println!("hit_point: {:?}, starting_point {:?}", hit_point.extend(1.0), starting_point);
			commands.spawn(SpriteBundle {
		        sprite: Sprite {
		            color: Color::VIOLET,
		            ..Default::default()
	        },
		        transform: Transform {
					translation: starting_point.extend(0.0),
					scale: Vec3::splat(15.0),
					..Default::default()
				},
				..Default::default()
		    })
			.insert(LaserAnimation {
		        timer: Timer::from_seconds(1.0, TimerMode::Once),
		        starting_point: starting_point.extend(0.0),
		        hit_point: hit_point.extend(0.0),
		    });
				
			    take_damage(
					commands,	
					&mut entity,
					health,
					20,
					//For some reason the x and y coordinate are in the wrong place, that is why
					//change them back for better.
					Vec3::new(hit_point.y, hit_point.x, 1.0), 
					death_sprite_animation,
				);

				
			}
			
		}
		else {
			// Shot missed
		}
		
		
	}
	
	
}
