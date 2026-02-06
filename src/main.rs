use bevy::prelude::*;
use bevy::window::WindowResolution;
use components::*;
use systems::movement::move_horses;
use systems::camera::*;
mod components;
mod resources;
mod systems;
mod ui;
mod utils;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d).insert(Projection::Orthographic({
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
                speed: 100.0,
                stamina: 10.0,
                power: 10.0,
                guts: 10.0,
                wit: 10.0,
            },
            RaceState {
                distance_traveled: 0.0,
                lane_index: 0,
                current_speed: 400.0,
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
                speed: 100.0,
                stamina: 10.0,
                power: 10.0,
                guts: 10.0,
                wit: 10.0,
            },
            RaceState {
                distance_traveled: 0.0,
                lane_index: 1,
                current_speed: 410.0,
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
        .add_systems(Update, move_horses)
		.add_systems(Update, (camera_follow, camera_switching))
        .run();
}
