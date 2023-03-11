mod player;
mod attack;
mod enemy;
mod sprite_animation;
mod enemy_util;
mod user_interface;


use bevy_rapier2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

// The state of the app so the game knows if you are in a game or in the main menu. Or getting roasted cuss you game-overed.
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
    // Does lots of setups and adds all the systems from the custom plugins.
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Smog".to_owned(),
                    resizable: false,
                    resolution: WindowResolution::new(1024.0, 1000.0).with_scale_factor_override(0.9),
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




/// set up a simple 2D camera
fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
