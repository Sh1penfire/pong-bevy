use bevy::prelude::*;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct PongBall;

#[derive(Component)]
pub struct Side{
    pub left: bool
}
#[derive(Component)]
pub struct Hitbox{
    pub area: Rect<f32>
}

#[derive(Component)]
pub struct Velocity{
    pub x: f32,
    pub y: f32,
    pub bound: bool
}

impl Velocity{
    pub fn set(&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }
}