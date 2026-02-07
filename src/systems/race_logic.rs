use crate::components::*;
use bevy::prelude::*;

// THIS WHOLE FILE SUCKS DO NOT READ THIS LMAO
// ALL MATH TAKEN FROM THE WIKI OF THE ACTUAL GAME SO THERES A BILLION MAGIC NUMBERS SORRY

const COURSE_DISTANCE: f32 = 2000.0;

fn get_base_speed(distance: f32) -> f32 {
    20.0 - (distance - 2000.0) / 1000.0
}

fn get_distance_type(distance: f32) -> DistanceType {
    if distance <= 1400.0 {
        DistanceType::Sprint
    } else if distance <= 1800.0 {
        DistanceType::Mile
    } else if distance <= 2400.0 {
        DistanceType::Medium
    } else {
        DistanceType::Long
    }
}

pub fn update_racer_stats(
    time: Res<Time>,
    mut query: Query<(&BaseStats, &mut RaceState, &RunStrategy, &DistanceAptitude)>,
) {
    let base_speed = get_base_speed(COURSE_DISTANCE);
    let middle_start = 100.0;
    let final_leg_start = COURSE_DISTANCE * 0.66;

    for (stats, mut state, strategy, aptitude) in query.iter_mut() {
        // determine phase
        if state.distance_traveled < middle_start {
            state.phase = RacePhase::Start;
        } else if state.distance_traveled < final_leg_start {
            state.phase = RacePhase::Middle;
        } else {
            state.phase = RacePhase::LastSpurt;
        }

        // get aptitude multi
        let dist_type = get_distance_type(COURSE_DISTANCE);
        let aptitude_grade = match dist_type {
            DistanceType::Sprint => aptitude.sprint,
            DistanceType::Mile => aptitude.mile,
            DistanceType::Medium => aptitude.medium,
            DistanceType::Long => aptitude.long,
        };
        let aptitude_mod = aptitude_grade.value();

        // hp drain
        if state.current_stamina > 0.0 {
            // 20 * (speed - base + 12)^2 / 144
            let speed_delta = state.current_speed - base_speed + 12.0;
            let drain_rate = 20.0 * speed_delta.powf(2.0) / 144.0;

			if aptitude_mod >= 1.0 {
				state.current_stamina -= drain_rate * time.delta_secs();
			}
			else {
				state.current_stamina -= drain_rate * (3.0 * (1.0 - aptitude_mod)) * time.delta_secs();
			}
        }

        // calculate base target speed using strategy coef
        let strategy_mod = get_strategy_modifier(*strategy, state.phase);
        let stat_bonus = 1.0 + (stats.speed / 1200.0) * 0.25;
        let mut target_speed = base_speed * stat_bonus * strategy_mod * aptitude_mod;

        // last spurt
        if state.phase == RacePhase::LastSpurt {
            if state.current_stamina > (stats.stamina * 0.1) {
                let speed_power_bonus = (500.0 * stats.speed).sqrt() * 0.003;

                // end closer speed
                let mut spurt_multiplier = 5.0;
                if *strategy == RunStrategy::EndCloser {
                    spurt_multiplier = 7.0;
                }

                target_speed += speed_power_bonus * spurt_multiplier;
            }
            // if no stamina it stays at (base_speed * strategy_mod)
        }

        // fatigue
        if state.current_stamina <= 0.0 {
            // tazunas favorite stat guts
            let guts_bonus = (200.0 * stats.guts).sqrt() * 0.001;
            target_speed = (base_speed * 0.85) + guts_bonus;
        }

        // acceleration
        let power_factor = (500.0 * stats.power).sqrt();
        let mut acceleration = 0.0002 * power_factor * aptitude_mod;

        // dash bonus
        let dash_threshold = base_speed * strategy_mod * 0.85;
        if state.phase == RacePhase::Start && state.current_speed < dash_threshold {
            acceleration += 24.0;
        }

        // last spurt acceleration bonus
        if state.phase == RacePhase::LastSpurt && state.current_stamina > 0.0 {
            acceleration *= 1.5;
        }

        // apply physics
        if state.current_speed < target_speed {
            state.current_speed += acceleration * time.delta_secs();
            state.current_speed = state.current_speed.min(target_speed);
        } else {
            state.current_speed -= 1.0 * time.delta_secs(); // decelerate
            state.current_speed = state.current_speed.max(target_speed);
        }

        // cap max speed just in case
        state.current_speed = state.current_speed.min(30.0);
    }
}

fn get_strategy_modifier(strategy: RunStrategy, phase: RacePhase) -> f32 {
    match (strategy, phase) {
        // frontrunner - fast start, slow end
        (RunStrategy::FrontRunner, RacePhase::Start) => 1.10,
        (RunStrategy::FrontRunner, RacePhase::Middle) => 1.00,
        (RunStrategy::FrontRunner, RacePhase::LastSpurt) => 0.96,

        // pace chaser - balanced
        (RunStrategy::PaceChaser, RacePhase::Start) => 0.98,
        (RunStrategy::PaceChaser, RacePhase::Middle) => 0.99,
        (RunStrategy::PaceChaser, RacePhase::LastSpurt) => 1.01,

        // late surger - slow start, solid end
        (RunStrategy::LateSurger, RacePhase::Start) => 0.95,
        (RunStrategy::LateSurger, RacePhase::Middle) => 1.00,
        (RunStrategy::LateSurger, RacePhase::LastSpurt) => 1.02,

        // end closer - slow start, fast end
        (RunStrategy::EndCloser, RacePhase::Start) => 0.90,
        (RunStrategy::EndCloser, RacePhase::Middle) => 0.98,
        (RunStrategy::EndCloser, RacePhase::LastSpurt) => 1.05,
    }
}

pub fn horse_ai_logic(
    time: Res<Time>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut RaceState, &RunStrategy)>,
        Query<(Entity, &RaceState)>,
    )>,
) {
    // collect other horses data to avoid borrow checker screaming at me
    let others: Vec<(Entity, f32, f32)> = param_set
        .p1()
        .iter()
        .map(|(entity, state)| (entity, state.distance_traveled, state.lane_position))
        .collect();

    for (my_entity, mut my_state, _strategy) in param_set.p0().iter_mut() {
        let mut blocked_forward = false;
        let mut blocked_inside = false;
        let mut blocked_outside = false;

        for (other_entity, other_distance, other_lane) in &others {
            if my_entity == *other_entity {
                continue;
            } // dont check self

            // relative distance (positive = ahead, negative = behind)
            let dist_diff = other_distance - my_state.distance_traveled;
            // relative lane (positive = outside, negative = inside)
            let lane_diff = other_lane - my_state.lane_position;

            // check ahead by a bit
            if dist_diff > 0.0 && dist_diff < 3.0 && lane_diff.abs() < 0.8 {
                blocked_forward = true;
            }

            // check sides
            if dist_diff.abs() < 2.0 {
                // check inside
                if (other_lane - (my_state.lane_position - 1.0)).abs() < 0.8 {
                    blocked_inside = true;
                }
                // check outside
                if (other_lane - (my_state.lane_position + 1.0)).abs() < 0.8 {
                    blocked_outside = true;
                }
            }
        }

        // DECISIONS
        // all horses want to be on the inside
        let mut desired_lane = 0.0;

        if blocked_forward {
            // priority 1: overtake on inside
            if !blocked_inside && my_state.lane_position > 0.5 {
                desired_lane = my_state.lane_position - 1.0;
            }
            // priority 2: overtake on outside
            else if !blocked_outside {
                desired_lane = my_state.lane_position + 1.0;
            }
            // priority 3: trapped so just wait
            else {
                desired_lane = my_state.lane_position;
            }
        } else {
            // path is clear
            // gradually drift towards rail
            if !blocked_inside {
                desired_lane = 0.0;
            } else {
                desired_lane = my_state.lane_position; // hold
            }
        }

        my_state.target_lane = desired_lane;

        // STEERING
        // 0.8 lanes per second
        let steer_speed = 0.2 * time.delta_secs();

        if my_state.lane_position < my_state.target_lane {
            my_state.lane_position += steer_speed;
            // clamp to avoid overshooting
            my_state.lane_position = my_state.lane_position.min(my_state.target_lane);
        } else if my_state.lane_position > my_state.target_lane {
            my_state.lane_position -= steer_speed;
            my_state.lane_position = my_state.lane_position.max(my_state.target_lane);
        }

        // hard clamp to track limits 0-12
        my_state.lane_position = my_state.lane_position.clamp(0.0, 12.0);
    }
}
