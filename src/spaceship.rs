use bevy::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};


const STARTING_VELOCITY: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_RADIUS: f32 = 5.0;
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;
pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        // use PostStartup, make sure Spawn take place after load assets
        app.add_systems(PostStartup, spawn_spaceship)
        
        .add_systems(Update, spaceship_movement_controls)
        .add_systems(Update, spaceship_weapon_controls);
    }
}


fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            position: Transform::from_translation(STARTING_TRANSLATION),
            velocity : Velocity {
                value: STARTING_VELOCITY,
            },
            collider: Collider::new(SPACESHIP_RADIUS),
            acceleration: Acceleration {value: Vec3::ZERO,},
            model : SceneRoot(
                scene_assets.spaceship.clone(),
            ),
        },
        SpaceShip,
    ));
}


fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<SpaceShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>, //Bevy Rename Input to ButtonInput since 0.12 -> 0.13
    time: Res<Time>,
){
    // Bevy 0.16 add error handling
    if let Ok((mut transform, mut velocity)) = query.single_mut() {
        let mut rotation = 0.0;
        let mut roll = 0.0;
        let mut movement = 0.0;

        // Bevy 0.12->-0.13 Rename KeyCode::W -> KeyCode::KeyW, represent physical keys on the keyboard
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement = -SPACESHIP_SPEED;
        } else if keyboard_input.pressed(KeyCode::KeyW) {
            movement = SPACESHIP_SPEED;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation = -SPACESHIP_ROTATION_SPEED * time.delta_secs();
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            rotation = SPACESHIP_ROTATION_SPEED * time.delta_secs();
        }

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            roll = -SPACESHIP_ROLL_SPEED * time.delta_secs();
        } else if keyboard_input.pressed(KeyCode::ControlLeft) {
            roll = SPACESHIP_ROLL_SPEED * time.delta_secs();
        }

        transform.rotate_y(rotation);
        transform.rotate_local_z(roll);
        velocity.value = -transform.forward() * movement;
    }
}


fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<SpaceShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single().unwrap();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                position: Transform::from_translation(transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneRoot(
                    scene_assets.missiles.clone(),
                ),
            },
            SpaceshipMissile,
        ));
    }
}