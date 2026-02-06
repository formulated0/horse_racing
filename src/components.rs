use bevy::prelude::*;

#[derive(Component)]
pub struct Horse {
}

#[derive(Component)]
pub struct BaseStats {
	pub speed: f32,
	pub stamina: f32,
	pub power: f32,
	pub guts: f32,
	pub wit: f32
}

#[derive(Component)]
pub struct Velocity {
	pub value: Vec3
}