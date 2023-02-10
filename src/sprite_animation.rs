use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(SpriteAnimationPlugin::setup)
            .add_system(SpriteAnimationPlugin::animate_sprite)
        ;
    }
}


impl SpriteAnimationPlugin {
    
    fn setup(
    
    ) {
    
    }
    
    
    fn animate_sprite(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            Entity,
        )>,
    ) {
        for (indices, mut timer, mut sprite, entity) in &mut query {
            timer.tick(time.delta());
            if timer.finished() {
                sprite.index = if sprite.index == indices.last {
                    commands.entity(entity).despawn_recursive();
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }

    
}




#[derive(Component, Clone, Copy, Resource)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

