use bevy::prelude::*;

pub const STRAIGHT_LENGTH: f32 = 800.0;
pub const TURN_RADIUS: f32 = 200.0;
pub const LANE_WIDTH: f32 = 30.0;

pub fn get_track_position(distance: f32, lane: usize) -> Vec3 {
    // radius for specific horse only
    let radius = TURN_RADIUS + (lane as f32 * LANE_WIDTH);

    // how long one lap is for THIS SPECIFIC LANE
    let curve_length = std::f32::consts::PI * radius;
    let lap_length = (2.0 * STRAIGHT_LENGTH) + (2.0 * curve_length);

    // position on current lap
    let current_dist = distance % lap_length;

    let x: f32;
    let y: f32;

    // TOP STRAIGHT
    if current_dist < STRAIGHT_LENGTH {
        x = current_dist;
        y = radius;
    }
    // TURN 1
    else if current_dist < STRAIGHT_LENGTH + curve_length {
        let curve_progress = current_dist - STRAIGHT_LENGTH; // calc how far into the curve we are
        let angle = (curve_progress / curve_length) * std::f32::consts::PI;

        x = STRAIGHT_LENGTH + (radius * angle.sin());
        y = radius * angle.cos();
    }
    // BOTTOM STRAIGHT
    else if current_dist < (2.0 * STRAIGHT_LENGTH) + curve_length {
        let straight_progress = current_dist - (STRAIGHT_LENGTH + curve_length);

        y = -radius;
        x = STRAIGHT_LENGTH - straight_progress;
    }
    // TURN 2
    else {
        let curve_progress = current_dist - ((STRAIGHT_LENGTH * 2.0) + curve_length);
        let angle = (curve_progress / curve_length) * std::f32::consts::PI;

        x = -(radius * angle.sin());
        y = -radius * angle.cos();
    }

    Vec3::new(x, y, 0.0)
}
