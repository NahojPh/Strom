mod player;
mod attack;

use bevy_rapier2d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(RapierDebugRenderPlugin {
            enabled: true,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugin(player::PlayerPlugin)
        .run();
}




/// set up a simple 2D scene
fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}


fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        window.set_cursor_grab_mode(CursorGrabMode::Confined);

        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        // also hide the cursor
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}// IDEAS
// -----------
// Ett spel där man snurrar runt ett skepp och måste hitta alla "fel" 
// som kommer upp och fixa dem genom att trycka på rätt tangentbords kanpp.
// Att vara snabb är viktigt eftersom skeppet kommer explordera om man inte är snabb nog



// Top down-turnbased spel på 3d plattform kanske med fysik i am not sure men med olika attacker.
