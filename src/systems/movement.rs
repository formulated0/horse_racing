use bevy::prelude::*;
use crate::components::Velocity;

pub fn move_horses(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in query.iter_mut() {
		transform.translation += velocity.value * time.delta_secs();
	}
}