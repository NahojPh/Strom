use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


use crate::{attack::{Attack, DeathSpriteAnimation, Health, Alive}, AppState};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(PlayerPlugin::setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((
                PlayerPlugin::handle_keyboard_input,
                PlayerPlugin::update_laser_attack_timer
            ).in_set(OnUpdate(AppState::InGame)))
            .add_system(PlayerPlugin::clean_on_exit.in_schedule(OnExit(AppState::InGame)))
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
    pub effects: Vec<Effect>,
    pub level: usize,
}

pub enum Effect {
    Slowness(f32), //f32 is multiplier
    Weakness(f32), //f32 is multiplier
    Strength(f32), //f32 is multiplier
    
}


#[derive(Component, Deref, DerefMut)]
struct LaserAttackTimer(Timer);



impl PlayerPlugin {
    
    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {

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
            effects: Vec::new(),
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
        .insert(LaserAttackTimer(Timer::from_seconds(1.0, TimerMode::Once)))
        ;
    
    }

    fn clean_on_exit(
        mut commands: Commands,
        query: Query<Entity, With<Player>>,
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

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
                                transform.rotation.to_axis_angle().0.truncate(),
                                &death_sprite_animation,
                                &mut health,
                            );  
                            laser_attack_timer.0.reset();                 
                        }
                    
                    },
                
                    _ => {}
                }
            }
            // eprintln!("Transform: {}", transform.translation.x);
            // eprintln!("Forced! {}", velocity.linvel);
        } 
    }

}

