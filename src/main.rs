mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*; 
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceShipPlugin;
use camera::CameraPlugin;



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        //user config plugins
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceShipPlugin)
        .add_plugins(CameraPlugin)
        .run();
}