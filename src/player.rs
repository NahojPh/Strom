use bevy::{prelude::*, input::mouse::MouseMotion};


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub sensitivity: f32,
}
// Consider using this instead of making your own camera controller
// https://github.com/sburris0/bevy_flycam

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(handle_keyboard_input)
            .add_system(handle_mouse_motion_input)
        ;
    }
}

fn setup(
    
) {
    
}


fn handle_keyboard_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        for key in input.get_pressed() {
            match key {
                KeyCode::W => {
                    transform.translation.z -= player.speed * time.delta_seconds(); 
                    
                },
                KeyCode::A => {
                    transform.translation.x -= player.speed * time.delta_seconds(); 
                },
                KeyCode::S => {
                    transform.translation.z += player.speed * time.delta_seconds(); 
                },
                KeyCode::D => {
                    transform.translation.x += player.speed * time.delta_seconds(); 
                },
                _ => {}
            }
        }
    } 
}

fn handle_mouse_motion_input(
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    
    for (mut transform, player) in query.iter_mut() {    
        for mouse_motion in motion_evr.iter() {
            transform.rotation.x -= mouse_motion.delta.y * player.sensitivity * time.delta_seconds();
            transform.rotation.y -= mouse_motion.delta.x * player.sensitivity * time.delta_seconds();
        }
    
    }
    
}