use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

use std::ops::Range;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)] 
pub struct Asteroid; // a marker component

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids, handle_asteroid_collisions),
        );
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    
    let translation = Vec3::new(rng.gen_range(SPAWN_RANGE_X),
    0.,
    rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            position : Transform::from_translation(translation),
            acceleration: Acceleration::new(acceleration),
            velocity: Velocity::new(velocity), // random generated v and a
            model: SceneRoot(
                scene_assets.asteroid.clone(),
            ),
            collider: Collider::new(RADIUS),
        },
        Asteroid,
    ));
}


fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn the asteroid.
            // Bevy 0.15 -> 0.16
            // 0.15
            // commands.entity(parent).despawn_recursive();
            // commands.entity(parent).despawn_descendants();
            // commands.entity(parent).despawn();

            // // 0.16
            // commands.entity(parent).despawn();
            // commands.entity(parent).despawn_related::<Children>();
            // commands.entity(parent).remove::<Children>().despawn();
            commands.entity(entity).despawn();
        }
    }
}