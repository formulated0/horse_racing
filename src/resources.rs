use crate::components::DistanceType;
use bevy::prelude::*;
use crate::components::*;
use bevy::color::palettes::basic;
use bevy::color::palettes::css;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Loading,
    #[default]
    TrackLengthSelect,
	TrackList,
    HorseSelect,
    Betting,
    Racing,
    Results,
}

#[derive(Resource, Default)]
pub struct RaceConfig {
    pub category: DistanceType,
    pub specific_track_name: String,
    pub length: f32,
}

#[derive(Clone)]
pub struct HorseTemplate {
    pub name: String,
    pub color: Color,
    pub stats: BaseStats,
    pub strategy: RunStrategy,
    pub aptitudes: DistanceAptitude,
}

impl HorseTemplate {
    pub fn get_all() -> Vec<Self> {
        vec![
            HorseTemplate {
                name: "Special Week".to_string(),
                color: basic::MAROON.into(),
                stats: BaseStats { speed: 1100.0, stamina: 1100.0, power: 1000.0, guts: 900.0, wit: 800.0 },
                strategy: RunStrategy::LateSurger,
                aptitudes: DistanceAptitude { sprint: AptitudeGrade::E, mile: AptitudeGrade::A, medium: AptitudeGrade::S, long: AptitudeGrade::A },
            },
            HorseTemplate {
                name: "Silence Suzuka".to_string(),
                color: basic::GREEN.into(),
                stats: BaseStats { speed: 1200.0, stamina: 600.0, power: 800.0, guts: 400.0, wit: 1000.0 },
                strategy: RunStrategy::FrontRunner,
                aptitudes: DistanceAptitude { sprint: AptitudeGrade::A, mile: AptitudeGrade::S, medium: AptitudeGrade::B, long: AptitudeGrade::E },
            },
            HorseTemplate {
                name: "Gold Ship".to_string(),
                color: basic::GRAY.into(),
                stats: BaseStats { speed: 900.0, stamina: 1200.0, power: 1200.0, guts: 1100.0, wit: 300.0 },
                strategy: RunStrategy::EndCloser,
                aptitudes: DistanceAptitude { sprint: AptitudeGrade::G, mile: AptitudeGrade::C, medium: AptitudeGrade::A, long: AptitudeGrade::S },
            },
            HorseTemplate {
                name: "Vodka".to_string(),
                color: css::PINK.into(),
                stats: BaseStats { speed: 1000.0, stamina: 700.0, power: 1100.0, guts: 600.0, wit: 600.0 },
                strategy: RunStrategy::LateSurger,
                aptitudes: DistanceAptitude { sprint: AptitudeGrade::B, mile: AptitudeGrade::S, medium: AptitudeGrade::B, long: AptitudeGrade::F },
            },
        ]
    }
}

#[derive(Resource, Default)]
pub struct SelectedHorses(pub Vec<usize>); 