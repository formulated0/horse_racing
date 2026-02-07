use bevy::prelude::*;
use bevy::window::WindowResolution;
use components::*;
use systems::movement::move_horses;
use systems::camera::*;
use systems::race_logic::update_racer_stats;
use ui::hud::*;
mod components;
mod resources;
mod systems;
mod ui;
mod utils;

fn setup(mut commands: Commands) {
    commands
		.spawn(Camera2d)
		.insert(Projection::Orthographic({
			let mut proj = OrthographicProjection::default_2d();
			proj.scale = 0.8;
			proj
    }));

    commands
        .spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
        ))
        .insert((
            Horse {},
            BaseStats {
                speed: 1000.0,
                stamina: 600.0,
                power: 900.0,
                guts: 10.0,
                wit: 10.0,
            },
            RaceState {
                distance_traveled: 0.0,
                lane_index: 0,
                current_speed: 0.0,
				current_stamina: 2480.0,
				phase: Start,
            },
            HorseNumber(0),
            PlayerFocus {},
        ));

    commands
        .spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
        ))
        .insert((
            Horse {},
            BaseStats {
                speed: 400.0,
                stamina: 300.0,
                power: 300.0,
                guts: 10.0,
                wit: 10.0,
            },
            RaceState {
                distance_traveled: 0.0,
                lane_index: 1,
                current_speed: 0.0,
				current_stamina: 2240.0,
				phase: Start,
            },
            HorseNumber(1),
        ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "uma!!".to_string(),
                resolution: WindowResolution::new(1280, 720),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
		.add_systems(Startup, setup_hud)
        .add_systems(Update, (update_racer_stats, move_horses).chain())
		.add_systems(Update, (camera_follow, camera_switching))
		.add_systems(Update, update_hud)
        .run();
}
