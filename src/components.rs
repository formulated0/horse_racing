use bevy::prelude::*;

#[derive(Component)]
pub struct Horse {}

#[derive(Component)]
pub struct HorseName(pub String);

#[derive(Component)]
pub struct BaseStats {
    pub speed: f32,
    pub stamina: f32,
    pub power: f32,
    pub guts: f32,
    pub wit: f32,
}

#[derive(Component)]
pub struct RaceState {
    pub distance_traveled: f32,
    pub lane_position: f32,
    pub target_lane: f32,
    pub current_speed: f32,
    pub current_stamina: f32,
    pub phase: RacePhase,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStrategy {
    FrontRunner,
    PaceChaser,
    LateSurger,
    EndCloser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DistanceType {
	#[default]
    Sprint, // < 1400
    Mile,   // 1401 - 1800
    Medium, // 1801 - 2400
    Long,   // > 2400
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AptitudeGrade {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl AptitudeGrade {
    pub fn value(&self) -> f32 {
        match self {
            Self::S => 1.05,
            Self::A => 1.00,
            Self::B => 0.90,
            Self::C => 0.80,
            Self::D => 0.60,
            Self::E => 0.40,
            Self::F => 0.20,
            Self::G => 0.10,
        }
    }
}

#[derive(Component)]
pub struct DistanceAptitude {
    pub sprint: AptitudeGrade,
    pub mile: AptitudeGrade,
    pub medium: AptitudeGrade,
    pub long: AptitudeGrade,
}

#[derive(Component)]
pub struct OnMainMenuScreen {}

#[derive(Component)]
pub struct TrackButton(pub DistanceType);

pub struct TrackMetadata {
    pub name: &'static str,
    pub length: f32,
}

impl DistanceType {
    pub fn get_tracks(&self) -> Vec<TrackMetadata> {
        match self {
            DistanceType::Sprint => vec![
                TrackMetadata { name: "Niigata Straight", length: 1000.0 },
                TrackMetadata { name: "Chukyo", length: 1200.0 },
            ],
            DistanceType::Mile => vec![
                TrackMetadata { name: "Tokyo", length: 1600.0 },
                TrackMetadata { name: "Hanshin", length: 1600.0 },
            ],
            DistanceType::Medium => vec![
                TrackMetadata { name: "Satsuki Sho", length: 2000.0 },
                TrackMetadata { name: "Tokyo (Derby)", length: 2400.0 },
                TrackMetadata { name: "Hanshin (Takarazuka)", length: 2200.0 },
            ],
            DistanceType::Long => vec![
                TrackMetadata { name: "Arima Kinen", length: 2500.0 },
                TrackMetadata { name: "Kikuka Sho", length: 3000.0 },
                TrackMetadata { name: "Tenno Sho (Spring)", length: 3200.0 },
            ],
        }
    }
}