use bevy::prelude::*;

use crate::components::{Velocity, PongBall, Hitbox};

pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_balls);
    }
}
fn spawn_balls(mut commands: Commands){
    let radius: f32 = 15.;
    commands.spawn_bundle(SpriteBundle{
        transform: Transform{
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
        sprite:  Sprite{
            color: Color::Rgba{
                    red: (1.), green: (1.), blue: (1.), alpha: (1.),
            },
            custom_size: Some(Vec2::new(radius, radius)),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Velocity{
        x: 5.,
        y: 0.,
        bound: false
    }).insert(PongBall{})
    .insert(Hitbox{
        area:  Rect { left: -radius/2., right: radius/2., top: radius/2., bottom: -radius/2. }
    });
}
