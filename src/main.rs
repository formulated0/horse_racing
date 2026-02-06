use bevy::prelude::*;
use components::*;
use systems::movement::move_horses;
mod components;
mod systems;
mod ui;
mod utils;
mod resources;

fn setup(mut commands: Commands) {
	commands.spawn(
		Camera2d::default()
	);
	commands.spawn((
		Sprite {color: Color::WHITE, custom_size: Some(Vec2::new(30.0,30.0)), ..default()},
		Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0))
	)).insert((
			Horse {}, BaseStats { speed: 100.0, stamina: 10.0, power: 10.0, guts: 10.0, wit: 10.0 }, Velocity { value: Vec3::new(100.0, 0.0, 0.0) }
	));
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).add_systems(Update, move_horses).run();
}
