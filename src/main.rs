mod player;
mod attack;
mod enemy;
mod sprite_animation;


use iyes_loopless::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
    Shopping,
}


fn main() {
    App::new()
        .add_loopless_state(AppState::InGame)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
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
