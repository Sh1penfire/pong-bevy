use bevy::{prelude::*};

use crate::{WinSize, components::*};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_paddles);
    }
}

fn spawn_paddles(mut commands: Commands, winsize: Res<WinSize>){
    let right: f32 = winsize.w/2.;
    let left = -right;

    let positions = [left, right];
    let mut i = 0;
    let width: f32 = 30.;
    let height: f32 = 150.;

    while i < 2 {
        commands.spawn_bundle(SpriteBundle{
            transform: Transform{
                translation: Vec3::new(positions[i], 0., 0.),
                ..Default::default()
            },
            sprite:  Sprite{
                color: Color::Rgba{
                        red: (1.), green: (1.), blue: (1.), alpha: (1.),
                },
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Player{
        }).insert(Velocity{x: 0., y: 1., bound: true})
        .insert(Hitbox{
            area: Rect{
                left: -width/2.,
                right: width/2.,
                top: height/2.,
                bottom:-height/2.
            }
        }).
        insert(Side{
            left: i == 0
        });
        i += 1;
    }
    debug!("spawned paddles");
}