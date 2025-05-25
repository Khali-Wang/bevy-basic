mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;

use bevy::prelude::*; 
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceShipPlugin;
use camera::CameraPlugin;
use asteroids::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;


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
        .add_plugins(AsteroidPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(CameraPlugin)
        .run();
}