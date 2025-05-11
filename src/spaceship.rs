use bevy::prelude::*;
use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,0.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0,0.0,1.0);


pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}


fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let gltf_handle = asset_server.load("assets/animal/Shark.glb#mesh");
    commands.spawn((
        Transform::from_translation(STARTING_TRANSLATION),
        Velocity {
            value: STARTING_VELOCITY,
        },
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("dragon\\DragonRigged.glb")),
        ),
    ));
}
