mod player;



use bevy_rapier3d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};

#[derive(Component)]
struct Block;

#[derive(Resource)]
struct Sign(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            enabled: true,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(cursor_grab_system)
        .add_plugin(player::PlayerPlugin)
        .insert_resource(Sign(1.0))
        // .add_system(upd)
        .run();
}


fn upd(
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Block>)>,
    block_query: Query<&Transform, With<Block>>,
    time: Res<Time>,
    mut sign: ResMut<Sign>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        for block_transform in block_query.iter() {
            if camera_transform.translation.x > 20.0 {
               // camera_transform.translation.x = 0.0;

                sign.0 = -1.0;

            }
            if camera_transform.translation.x < -20.0 {
                sign.0 = 1.0;
            }
            camera_transform.translation.x += 5.0 * time.delta_seconds() * sign.0;
            camera_transform.look_at(block_transform.translation, Vec3::Y);
        }
    }
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
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Torus { radius: 1.0, ring_radius: 0.., ..Default::default() })),
    //     material: materials.add(Color::AZURE.into()),
    //     transform: Transform::from_translation(Vec3::Y),
    //     ..Default::default()
    // });
    
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::BEIGE.into()),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(10.0, 10.0, 10.0))
    ;
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // })
    // .insert(Block)
    // .insert(RigidBody::Dynamic)
    // .insert(Collider::cuboid(10.0, 1.0, 10.0))
    // ;
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..default()
    })
    .insert(player::Player {
        speed: 10.0,
        sensitivity: 0.05,
    });
}


// IDEAS
// -----------
// Ett spel där man snurrar runt ett skepp och måste hitta alla "fel" 
// som kommer upp och fixa dem genom att trycka på rätt tangentbords kanpp.
// Att vara snabb är viktigt eftersom skeppet kommer explordera om man inte är snabb nog



// Top down-turnbased spel på 3d plattform kanske med fysik i am not sure men med olika attacker.
