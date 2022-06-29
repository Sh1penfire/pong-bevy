use bevy::prelude::*;
use systems::*;

mod paddle;
mod ball;
mod components;
mod systems;
pub struct WinSize{
    w: f32,
    h: f32
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Rgba{
            red: (0.), green: (0.), blue: (0.), alpha: (1.0)
        }))
        .insert_resource(WindowDescriptor{
            title: "Pong!".to_string(),
            width: 700.0,
            height: 700.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(paddle::PaddlePlugin)
        .add_plugin(ball::BallPlugin)
        .add_startup_system(setup)
        .add_system(move_all_velocity_objects)
        .add_system(move_all_players
            .after(move_all_velocity_objects)
        )
        .add_system(collision_ball
            .after(move_all_velocity_objects)
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    println!("Yay!");

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size: WinSize = WinSize{w: win_w, h: win_h};
    commands.insert_resource(win_size);
}
