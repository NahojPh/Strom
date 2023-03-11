use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;


use crate::{
    attack::{
        Attack,
        DeathSpriteAnimation,
        Health,
        Alive
    }, 
    AppState
};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(PlayerPlugin::setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((
                PlayerPlugin::handle_keyboard_input,
                PlayerPlugin::update_laser_attack_timer,
                PlayerPlugin::render_laser_attack_timer,
            ).in_set(OnUpdate(AppState::InGame)))
            .add_system(PlayerPlugin::clean_on_exit_in_game.in_schedule(OnExit(AppState::InGame)))
        ;
    }

}


#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub max_speed: f32,
    pub base_damage: f32,    
    pub attack_speed: f32,
    pub bullet_amount: usize,
    pub level: usize,
}



#[derive(Component, Deref, DerefMut)]
struct LaserAttackTimer(Timer);

#[derive(Component)]
struct LaserIcon;




impl PlayerPlugin {
    
    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
		window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
		let Ok(window) = window_query.get_single() else {
	        return;
	    };

        let space_ship_texture = asset_server.load("Overlord_02_119x161.png");

        commands.spawn(SpriteBundle {
            transform: Transform::default(),
            texture: space_ship_texture,
            ..Default::default()
        })
        .insert(Player {
            speed: 800.0,
            max_speed: 400.0,
            base_damage: 500.0,
            attack_speed: 1.0,
            bullet_amount: 1,
            level: 1,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
    	.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(Damping {
            linear_damping: 1.5,
            angular_damping: 20.0,
        })
        .insert(Health {
            health: 100,
            max_health: 100,
        })
        .insert(Alive)
        .insert(LaserAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        ;


    
    }

    // Adds and removes an icon when the laser is ready to be used.
    fn render_laser_attack_timer(
		window_query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
        laser_timer_query: Query<&LaserAttackTimer, (With<Player>, Without<LaserIcon>)>,
        laser_icon_query: Query<Entity, (With<LaserIcon>, Without<Player>)>,
        time: Res<Time>,
    ) {
        for laser_attack_timer in laser_timer_query.iter() {
            if laser_attack_timer.finished() {
        		let Ok(window) = window_query.get_single() else {
        	        return;
        	    };
                
                let laser_icon_texture = asset_server.load("LaserIcon.png");
                commands.spawn(SpriteBundle {
                    transform: Transform::from_xyz(
                        (window.width() / 2.0) * -1.0 + 60.0, 
                        (window.height() / 2.0) * -1.0 + 60.0,
                        100.0
                    ),
                    texture: laser_icon_texture,
                    ..Default::default()
                })
                .insert(LaserIcon)
                ;
            }
            else {
                for entity in laser_icon_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
    

    // Does some clean up when exiting the InGame state.
    fn clean_on_exit_in_game(
        mut commands: Commands,
        player_query: Query<Entity, With<Player>>,
        laser_icon_query: Query<Entity, (With<LaserIcon>, Without<Player>)>,
    ) {
        for entity in player_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in laser_icon_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Increments the laser attack timer.
    fn update_laser_attack_timer(
        time: Res<Time>,
        mut query: Query<&mut LaserAttackTimer>,
    ) {
        for mut laser_attack_timer in query.iter_mut() {
            if !laser_attack_timer.0.finished() {
                laser_attack_timer.0.tick(time.delta());
            }
        }
    
    }

    // Handles player movement and shooting.
    fn handle_keyboard_input(
        mut commands: Commands,
        input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Velocity, &Player, &Transform, &mut Health, &mut LaserAttackTimer)>,
        time: Res<Time>,
        rapier_context: Res<RapierContext>,
    	death_sprite_animation: Res<DeathSpriteAnimation>,
    
    
    ) {
        // dbg!("open");
        for (mut velocity, player, transform, mut health, mut laser_attack_timer) in query.iter_mut() {
            // dbg!("handle..");
            for key in input.get_pressed() {
            // dbg!("handle.. keys");
                match key {
                    KeyCode::W => {
                        if velocity.linvel.y < player.max_speed {
                            velocity.linvel.y += player.speed * time.delta_seconds();
                        }
                    },
                    KeyCode::A => {
                        if velocity.linvel.x.abs() < player.max_speed {
                            velocity.linvel.x -= player.speed * time.delta_seconds();
                        }
                    
                    },
                    KeyCode::S => {
                        if velocity.linvel.y.abs() < player.max_speed {
                            velocity.linvel.y -= player.speed * time.delta_seconds();
                        }
                    
                    },
                    KeyCode::D => {
                        if velocity.linvel.x < player.max_speed {
                            velocity.linvel.x += player.speed * time.delta_seconds();
                        }
                    },
                    KeyCode::F => {
                        if laser_attack_timer.finished() {
                            Attack::shot_laser(
                                &mut commands,
                                &rapier_context,
                                transform.translation.truncate(),
                                &death_sprite_animation,
                                &mut health,
                            );  
                            laser_attack_timer.0.reset();                 
                        }
                    
                    },
                    // Key press is not one of the aformentioned ones so its tossed aside.                
                    _ => {}
                }
            }
        } 
    }

}

