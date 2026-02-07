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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceType {
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