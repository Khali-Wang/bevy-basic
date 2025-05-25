use bevy::prelude::*;

use crate::collision_detection::Collider;
pub struct MovementPlugin;

#[derive(Debug, Component)]
pub struct Velocity {
   pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Debug, Component)]
pub struct Acceleration {
   pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub position: Transform,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub model: SceneRoot,
}


impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for(accelation, mut velocity) in query.iter_mut() {
        velocity.value += accelation.value * time.delta_secs();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for(velocity, mut position) in query.iter_mut() {
        position.translation += velocity.value * time.delta_secs();
    }
}