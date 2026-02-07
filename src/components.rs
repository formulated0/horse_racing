use bevy::prelude::*;

#[derive(Component)]
pub struct Horse {}

#[derive(Component)]
pub struct BaseStats {
	pub speed: f32,
	pub stamina: f32,
	pub power: f32,
	pub guts: f32,
	pub wit: f32
}

#[derive(Component)]
pub struct RaceState {
	pub distance_traveled: f32,
	pub lane_index: usize,
	pub current_speed: f32,
	pub current_stamina: f32,
	pub phase: RaceState,
}

#[derive(Component)]
pub struct HorseNumber(pub usize);

#[derive(Component)]
pub struct PlayerFocus {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RacePhase {
    Start,
    Middle,
    LastSpurt,
}