use bevy::{prelude::*, state::commands}; 



#[derive(Debug, Component)]
struct Velocity {
   pub value: Vec3,
}


fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Velocity {value: Vec3::new(1.0, 0.0 ,0.0)},
    ));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for(velocity, mut position) in query.iter_mut() {
        position.translation.x += velocity.value.x;
        position.translation.y += velocity.value.y;
        position.translation.z += velocity.value.z;
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for(entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, position);
    }
}