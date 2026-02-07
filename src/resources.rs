use crate::components::DistanceType;
use bevy::prelude::*;

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