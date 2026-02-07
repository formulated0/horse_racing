use crate::{GameState, components::*, resources::RaceConfig};
use bevy::{
    ecs::{query, spawn, system},
    prelude::*,
};
use std::thread::spawn;

#[derive(Component)]
pub struct SpecificTrackButton(pub f32, pub String);

pub fn setup_track_length_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .insert(OnMainMenuScreen {})
        .with_children(|parent| {
            parent.spawn((
                Text::new("TRACK LENGTH"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(button(&String::from("SPRINT")))
                        .insert(TrackButton(DistanceType::Sprint));
                    parent
                        .spawn(button(&String::from("MILE")))
                        .insert(TrackButton(DistanceType::Mile));
                    parent
                        .spawn(button(&String::from("MEDIUM")))
                        .insert(TrackButton(DistanceType::Medium));
                    parent
                        .spawn(button(&String::from("LONG")))
                        .insert(TrackButton(DistanceType::Long));
                });
        });
}

fn button(name: &String) -> impl Bundle {
    (
        Button,
        Node {
            width: px(200),
            height: px(200),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::WHITE),
        children![(
            Text::new(name),
            TextFont {
                font_size: 45.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.0, 0.0)),
        )],
    )
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn track_len_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &TrackButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut race_config: ResMut<RaceConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                race_config.category = button.0;
                next_state.set(GameState::TrackList);
                Color::srgb(1.0, 0.0, 0.0);
            }
            Interaction::Hovered => {
                Color::srgb(0.7, 0.7, 0.7);
            }
            Interaction::None => {
                Color::WHITE;
            }
        }
    }
}

pub fn setup_track_list(mut commands: Commands, race_config: Res<RaceConfig>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
        ))
        .with_children(|parent| {
            parent
				.spawn((
					Text::new(format!("SELECT {:?} TRACK", race_config.category)),
					TextFont {
						font_size: 60.0,
						..default()
					},
					TextColor(Color::WHITE),
					Node {
						margin: UiRect::bottom(Val::Px(50.0)),
						..default()
					},
				));
			parent
				.spawn((
					Node {
						width: Val::Percent(100.0),
						height: Val::Percent(100.0),
						flex_wrap: FlexWrap::Wrap,
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						..default()
					}
				))
				.with_children(|grid| {
					let tracks = race_config.category.get_tracks();
					for track in tracks {
						grid.spawn((
							Button,
							Node {
								width: px(160),
								height: px(80),
								border: UiRect::all(px(5)),
								justify_content: JustifyContent::Center,
								align_items: AlignItems::Center,
								..default()
							},
							children![(
								Text::new(format!("{}\n{:.0}m", track.name, track.length)),
								TextFont {
									font_size: 45.0,
									..default()
								},
								TextColor(Color::srgb(1.0, 0.0, 0.0)),
							)],
						)).insert(SpecificTrackButton(track.length, track.name.to_string())).insert(OnMainMenuScreen{});
					}
				});
        });
}

pub fn track_specific_button_interaction(
	mut interaction_query: Query<
        (&Interaction, &SpecificTrackButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut race_config: ResMut<RaceConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
	for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            race_config.length = button.0;
            race_config.specific_track_name = button.1.clone();
            next_state.set(GameState::HorseSelect);
        }
    }
}