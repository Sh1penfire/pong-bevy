use std::ops::Mul;

use bevy::prelude::*;
use bevy::math::*;

use crate::{components::{Velocity, Player, PongBall, Side, Hitbox}, WinSize};

pub fn move_all_velocity_objects(winsize: Res<WinSize>, mut query: Query<(&Velocity, &mut Transform)>){
    for(velocity, mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
        run_movement_tick(translation, velocity);
        if(translation.x > winsize.w/2.){translation.x = winsize.w/2.;}
        if(translation.x < -winsize.w/2.){translation.x = -winsize.w/2.;}

        if(translation.y > winsize.h/2.){translation.y = winsize.h/2.;}
        if(translation.y < -winsize.h/2.){translation.y = -winsize.h/2.;}
    }
}

pub fn move_all_players(keys: Res<Input<KeyCode>>, mut query: Query<(&Side, &mut Velocity, With<Player>)>){
    for(side, mut velocity, bool) in query.iter_mut(){
        velocity.y = 0.;
        if(side.left){
            if keys.pressed(KeyCode::W) {
                velocity.y += 5.;
            }
            else if keys.pressed(KeyCode::S) {
                velocity.y -= 5.;
            }
        }
        else{
            if keys.pressed(KeyCode::O) {
                velocity.y += 5.;
            }
            else if keys.pressed(KeyCode::L) {
                velocity.y -= 5.;
            }
        }
    }
}

pub fn collision_ball(
    mut ball_query: Query<(&Transform, &mut Velocity, &Hitbox, Without<Player>)>,
    player_query: Query<(&Transform, &Hitbox, Without<PongBall>)>,
    winsize: Res<WinSize>){

    let (transform, mut velocity, ball_hitbox, _bool) = ball_query.single_mut();

    let mut translation: Vec3 = transform.translation;

    for (tf, player_hitbox, _bool) in player_query.iter(){
        if(intersects(translation, ball_hitbox.area, tf.translation, player_hitbox.area)){
            let new_vel: Vec2 = Vec2::new(translation.x - tf.translation.x, translation.y - tf.translation.y);
            let normalized_vel = new_vel.normalize().mul(3.85);

            velocity.set(normalized_vel.x, normalized_vel.y);
            let mut i = 0;
            while i < 3{

                translation.y += velocity.y;
                translation.x += velocity.x;

                if(!intersects(translation, ball_hitbox.area, tf.translation, player_hitbox.area)){ break;};
                i += 1;
            }
        }
    }
    
    if(translation.y == -winsize.h/2. || translation.y == winsize.h/2.){
        let vel_x = velocity.x;
        let vel_y = velocity.y;
        velocity.set(vel_x, -vel_y);
    }
    if(translation.x == -winsize.w/2. || translation.x == winsize.w/2.){
        let new_vel: Vec2 = Vec2::new(-translation.x, -translation.y);
        let normalized_vel = new_vel.normalize().mul(3.85);
        velocity.set(normalized_vel.x, normalized_vel.y);
    }
}

//returns if the rects intersect
fn intersects(pos1: Vec3, rect1: Rect<f32>, pos2: Vec3, rect2: Rect<f32>) -> bool{
    
    //intersection on the x axis
    if rect1.left + pos1.x > rect2.right + pos2.x || rect1.right + pos1.x < rect2.left + pos2.x{
        return false;
    }
    //intersection on the y axis
    
    if rect1.bottom + pos1.y > rect2.top + pos2.y || rect1.top + pos1.y < rect2.bottom + pos2.y{
        return false;
    }
    true
}

//Runs a single movement tick on a point.
fn run_movement_tick(translation: &mut Vec3, velocity: &Velocity){
    translation.y += velocity.y;
    translation.x += velocity.x;
}