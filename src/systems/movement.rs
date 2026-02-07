use crate::components::RaceState;
use crate::utils::track_math::get_track_position;
use bevy::prelude::*;

pub fn move_horses(time: Res<Time>, mut query: Query<(&mut Transform, &mut RaceState)>) {
    for (mut transform, mut race_state) in query.iter_mut() {
        race_state.distance_traveled += race_state.current_speed * time.delta_secs();
        let new_pos = get_track_position(race_state.distance_traveled, race_state.lane_index);
        transform.translation = new_pos;
    }
}
