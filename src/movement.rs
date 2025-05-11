use bevy::prelude::*;
pub struct MovementPlugin;

#[derive(Debug, Component)]
pub struct Velocity {
   pub value: Vec3,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for(velocity, mut position) in query.iter_mut() {
        position.translation += velocity.value * time.delta_secs();
    }
}