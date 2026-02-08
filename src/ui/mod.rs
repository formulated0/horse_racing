use crate::GameState;
use crate::components::{self, OnMainMenuScreen};
use crate::resources::RaceConfig;
use crate::ui::menu::*;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::Commands;
use bevy::ecs::*;
use bevy::state::condition::in_state;
use bevy::state::state::{OnEnter, OnExit};
use bevy::ui::UiPlugin;
pub mod hud;
pub mod menu;

pub struct UiPluginStruct;

impl Plugin for UiPluginStruct {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::TrackLengthSelect),
            menu::setup_track_length_select,
        );
		app.add_systems(
			OnEnter(GameState::TrackList), 
			menu::setup_track_list
		);
		app.add_systems(
			OnEnter(GameState::HorseSelect),
			menu::setup_horse_select
		);
        app.add_systems(
            OnExit(GameState::TrackLengthSelect),
            menu::despawn_screen::<OnMainMenuScreen>,
        );
		app.add_systems(
            OnExit(GameState::TrackList),
            menu::despawn_screen::<OnMainMenuScreen>,
        );
		app.add_systems(
            Update,
            track_len_button_interaction.run_if(in_state(GameState::TrackLengthSelect)),
        );
		app.add_systems(
            Update,
            track_specific_button_interaction.run_if(in_state(GameState::TrackList)),
        );
		app.add_systems(
            Update,
			horse_select_interaction.run_if(in_state(GameState::HorseSelect))
		);
		app.add_systems(
            Update,
			start_button_interaction.run_if(in_state(GameState::HorseSelect))
		);
    }
}
