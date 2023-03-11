mod player;
mod attack;
mod enemy;
mod sprite_animation;
mod enemy_util;
mod user_interface;


use bevy_rapier2d::prelude::*;
use bevy::{prelude::*, window::{CursorGrabMode, PresentMode, WindowResolution}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default, SystemSet)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    Shopping,
    GameOver,
}


fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Smog".to_owned(),
                    resolution: WindowResolution::new(1024.0, 640.0).with_scale_factor_override(0.9),
                    ..Default::default()
                    
                }),
                ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            enabled: true,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(attack::AttackPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(user_interface::UiPlugin)
        .add_plugin(sprite_animation::SpriteAnimationPlugin)
        .run();
}




/// set up a simple 2D scene
fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}


// IDEAS
// -----------
// Ett spel där man snurrar runt ett skepp och måste hitta alla "fel" 
// som kommer upp och fixa dem genom att trycka på rätt tangentbords kanpp.
// Att vara snabb är viktigt eftersom skeppet kommer explordera om man inte är snabb nog



// Top down-turnbased spel på 3d plattform kanske med fysik i am not sure men med olika attacker.
