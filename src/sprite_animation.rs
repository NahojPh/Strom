use bevy::prelude::*;
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // This can always run so sprite animations can run anywere.
            .add_system(SpriteAnimationPlugin::animate_sprite) 
        ;
    }
}


impl SpriteAnimationPlugin {
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
        //To increment the sprite index and therefore
        //change the sprite texture when the timer is finished
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



// The size of the sprite sheet
#[derive(Component, Clone, Copy, Resource)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

// The timer about when to change texture
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

