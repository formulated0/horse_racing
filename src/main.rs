use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::WindowResolution;
use components::*;
use systems::camera::*;
use systems::movement::move_horses;
use systems::race_logic::update_racer_stats;
use ui::hud::*;
use utils::track_math::get_track_position;
use crate::systems::race_logic;
mod components;
mod resources;
mod systems;
mod ui;
mod utils;

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
        .add_systems(Update, (
			update_racer_stats, 
			race_logic::horse_ai_logic, 
			move_horses
		).chain())
        .add_systems(Update, (camera_follow, camera_switching))
        .add_systems(Update, update_hud)
        .run();
}

fn draw_track_rails(commands: &mut Commands) {
    let track_length = 2000.0 + (2.0 * std::f32::consts::PI * 200.0); // approx total length
    let step = 20.0; // draw a dot every 20m

    for i in 0..(track_length as i32 / step as i32) {
        let dist = i as f32 * step;

        // INNER RAIL
        let pos_inner = get_track_position(dist, 0.0);
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            Transform::from_translation(pos_inner),
        ));

        // OUTER RAIL
        let pos_outer = get_track_position(dist, 8.0);
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            Transform::from_translation(pos_outer),
        ));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    draw_track_rails(&mut commands);
    commands.spawn(Camera2d).insert(Projection::Orthographic({
        let mut proj = OrthographicProjection::default_2d();
        proj.scale = 0.8;
        proj
    }));

    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
            Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
        ))
        .insert((
            Horse {},
            HorseName("skibid".to_string()),
            BaseStats {
                speed: 1000.0,
                stamina: 600.0,
                power: 900.0,
                guts: 10.0,
                wit: 10.0,
            },
            RaceState {
                distance_traveled: 0.0,
                lane_position: 2.0,
                target_lane: 2.0,
                current_speed: 0.0,
                current_stamina: 2480.0,
                phase: RacePhase::Start,
            },
            HorseNumber(0),
            PlayerFocus {},
            RunStrategy::EndCloser,
            create_aptitude(DistanceType::Medium),
            Collider { radius: 15.0 },
        ));

    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
            Transform::from_translation(Vec3::new(-500.0, 0.0, 0.0)),
        ))
        .insert((
            Horse {},
            HorseName("JE_Vacation".to_string()),
            BaseStats {
                speed: 400.0,
                stamina: 300.0,
                power: 300.0,
                guts: 10.0,
                wit: 10.0,
            },
            RaceState {
                distance_traveled: 0.0,
                lane_position: 0.0,
                target_lane: 0.0,
                current_speed: 0.0,
                current_stamina: 2240.0,
                phase: RacePhase::Start,
            },
            HorseNumber(1),
            RunStrategy::FrontRunner,
            create_aptitude(DistanceType::Sprint),
            Collider { radius: 15.0 },
        ));
}

fn create_aptitude(best_dist: DistanceType) -> DistanceAptitude {
    match best_dist {
        DistanceType::Sprint => DistanceAptitude {
            sprint: AptitudeGrade::A,
            mile: AptitudeGrade::B,
            medium: AptitudeGrade::E,
            long: AptitudeGrade::G,
        },
        DistanceType::Mile => DistanceAptitude {
            sprint: AptitudeGrade::B,
            mile: AptitudeGrade::A,
            medium: AptitudeGrade::B,
            long: AptitudeGrade::E,
        },
        DistanceType::Medium => DistanceAptitude {
            sprint: AptitudeGrade::E,
            mile: AptitudeGrade::B,
            medium: AptitudeGrade::A,
            long: AptitudeGrade::B,
        },
        DistanceType::Long => DistanceAptitude {
            sprint: AptitudeGrade::G,
            mile: AptitudeGrade::E,
            medium: AptitudeGrade::B,
            long: AptitudeGrade::A,
        },
    }
}
