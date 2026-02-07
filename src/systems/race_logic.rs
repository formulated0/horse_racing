use crate::components::*;
use bevy::prelude::*;

// THIS WHOLE FILE SUCKS DO NOT READ THIS LMAO
// ALL MATH TAKEN FROM THE WIKI OF THE ACTUAL GAME SO THERES A BILLION MAGIC NUMBERS SORRY

const COURSE_DISTANCE: f32 = 2000.0;

fn get_base_speed(distance: f32) -> f32 {
	20.0 - (distance - 2000.0) / 1000.0
}


pub fn update_racer_stats(
    time: Res<Time>,
    mut query: Query<(&BaseStats, &mut RaceState)>
) {
    let base_speed = get_base_speed(COURSE_DISTANCE);
    let middle_start = 100.0;
    let final_leg_start = COURSE_DISTANCE * 0.66; 

    for (stats, mut state) in query.iter_mut() {
        // determine phase
        if state.distance_traveled < middle_start {
            state.phase = RacePhase::Start;
        } else if state.distance_traveled < final_leg_start {
            state.phase = RacePhase::Middle;
        } else {
            state.phase = RacePhase::LastSpurt;
        }

        // hp drain
        if state.current_stamina > 0.0 {
            // 20 * (speed - base + 12)^2 / 144
            let speed_delta = state.current_speed - base_speed + 12.0;
            let drain_rate = 20.0 * speed_delta.powf(2.0) / 144.0;
            state.current_stamina -= drain_rate * time.delta_secs();
        }

        // target speed based on phase
        let mut target_speed = base_speed;

        match state.phase {
            RacePhase::Start => {
                target_speed = base_speed * 1.05; 
            }
            RacePhase::Middle => {
                target_speed = base_speed * 1.0; 
            }
            RacePhase::LastSpurt => {
                if state.current_stamina > (stats.stamina * 0.1) {
					let speed_power_bonus = (500.0 * stats.speed).sqrt() * 0.002;
					target_speed = base_speed + (speed_power_bonus * 5.0); // multiplier so its noticeable
                } else {
                    // if no stamina just hold base speed
                    target_speed = base_speed;
                }
            }
        }

        // fatigue
        if state.current_stamina <= 0.0 {
            // tazunas favorite stat guts
            let guts_bonus = (200.0 * stats.guts).sqrt() * 0.001;
            target_speed = (base_speed * 0.85) + guts_bonus;
        }

        //  acceleration
        let power_factor = (500.0 * stats.power).sqrt();
        let mut acceleration = 0.0006 * power_factor;

        // start dash bonus
        if state.phase == RacePhase::Start && state.current_speed < base_speed * 0.85 {
            acceleration += 24.0;
        }

        // last spurt bonus
        if state.phase == RacePhase::LastSpurt && state.current_stamina > 0.0 {
            acceleration *= 1.5; 
        }

        // apply physics
        if state.current_speed < target_speed {
            state.current_speed += acceleration * time.delta_secs();
            state.current_speed = state.current_speed.min(target_speed);
        } else {
            state.current_speed -= 1.0 * time.delta_secs(); // Decelerate
            state.current_speed = state.current_speed.max(target_speed);
        }
        
		// cap max speed just in case
        state.current_speed = state.current_speed.min(30.0);
    }
}
