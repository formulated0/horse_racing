use crate::components::RaceState;
use crate::utils::track_math::get_track_position;
use bevy::prelude::*;

pub fn move_horses(time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut RaceState)>) {
    for (entity, mut transform, mut race_state) in query.iter_mut() {
        race_state.distance_traveled += race_state.current_speed * time.delta_secs();

        let unique_offset = entity.index().index() as f32;
		let wobble = (time.elapsed_secs() * 1.0 + unique_offset).sin() * 0.05;
		let visual_lane = race_state.lane_position + wobble;
		let new_pos = get_track_position(race_state.distance_traveled, visual_lane);
		let z_pos = 100.0 - new_pos.y * 0.01; 
        transform.translation = Vec3::new(new_pos.x, new_pos.y, z_pos);
    }
}
