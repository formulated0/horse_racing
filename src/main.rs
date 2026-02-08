use crate::resources::GameState;
use crate::resources::HorseTemplate;
use crate::resources::RaceConfig;
use crate::resources::SelectedHorses;
use crate::systems::race_logic;
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
        .add_plugins(ui::UiPluginStruct)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_racer_stats, race_logic::horse_ai_logic, move_horses).chain(),
        )
        .add_systems(Update, (camera_follow, camera_switching))
        .add_systems(Update, update_hud)
        .add_systems(
            OnEnter(GameState::Racing),
            (spawn_selected_horses, draw_track_rails),
        )
        .add_systems(Startup, ui::hud::setup_hud)
        .insert_resource(RaceConfig::default())
        .insert_resource(SelectedHorses::default())
        .run();
}

fn draw_track_rails(mut commands: Commands) {
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
    commands.spawn(Camera2d).insert(Projection::Orthographic({
        let mut proj = OrthographicProjection::default_2d();
        proj.scale = 0.8;
        proj
    }));
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

fn spawn_selected_horses(
    mut commands: Commands,
    selected: Res<SelectedHorses>,
    config: Res<RaceConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let templates = HorseTemplate::get_all();

    for (i, &horse_idx) in selected.0.iter().enumerate() {
        let horse = &templates[horse_idx];
        let start_lane = i as f32;
        let start_y = start_lane * 30.0;
        let max_hp = 0.8 * horse.stats.stamina + config.length;

        commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(15.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(horse.color))),
                Transform::from_translation(Vec3::new(-500.0, start_y, 0.0)),
            ))
            .insert((
                Horse {},
                HorseNumber(i),
                HorseName(horse.name.to_string().clone()),
                horse.stats.clone(),
                RaceState {
                    distance_traveled: 0.0,
                    lane_position: start_lane,
                    target_lane: start_lane,
                    current_speed: 0.0,
                    current_stamina: max_hp,
                    phase: RacePhase::Start,
                },
                horse.strategy.clone(),
                Collider { radius: 15.0 },
            ));

        if i == 0 {
            let entity_id = commands.spawn_empty().id();
            commands.entity(entity_id).insert(PlayerFocus {});
        }
    }
}
