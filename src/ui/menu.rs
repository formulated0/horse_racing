use crate::{
    GameState,
    components::*,
    resources::{HorseTemplate, RaceConfig, SelectedHorses},
};
use bevy::{
    color::palettes::css,
    ecs::{query, spawn, system},
    prelude::*, ui::Pressed,
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
            parent.spawn((
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
                .spawn(
                    (Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }),
                )
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
                        ))
                        .insert(SpecificTrackButton(track.length, track.name.to_string()))
                        .insert(OnMainMenuScreen {});
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

pub fn setup_horse_select(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            BackgroundColor(Color::Srgba(css::DARK_GRAY)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("SELECT HORSES")),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|grid| {
                    let horses = HorseTemplate::get_all();
                    for (index, horse) in horses.iter().enumerate() {
                        grid.spawn((
                            Button,
                            Node {
                                width: px(200.0),
                                height: Val::Auto,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(px(10.0)),
                                margin: UiRect::all(px(10.0)),
                                border: UiRect::all(px(2)),
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        ))
                        .insert(HorseSelectButton(index))
                        .insert(OnMainMenuScreen {})
                        .with_children(|card| {
                            card.spawn((
                                Text::new(&horse.name),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                            ));
                            card.spawn(
                                (Node {
                                    display: Display::None,
                                    flex_direction: FlexDirection::Column,
                                    margin: UiRect::top(px(10.0)),
                                    ..default()
                                }),
                            )
                            .insert(HorseStatsPanel {})
                            .with_children(|stats_box| {
                                let s = &horse.stats;
                                let text = format!(
                                    "SPD: {:.0}\nSTA: {:.0}\nPOW: {:.0}\nGUT: {:.0}\nWIT: {:.0}",
                                    s.speed, s.stamina, s.power, s.guts, s.wit
                                );
                                stats_box.spawn((
                                    Text::new(text),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::Srgba(css::GOLD)),
                                ));
                            });
                        });
                    }
                });
            parent
                .spawn((
                    Button,
                    Node {
                        width: px(200.0),
                        height: px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(50.0)),
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(css::GREY)),
                    Text::new("START RACE"),
                    TextFont {
                        font_size: 50.0,
                        ..default()
                    },
                ))
                .insert(StartRaceButton{});
        });
}

pub fn horse_select_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &HorseSelectButton,
            &Children,
            &mut BorderColor,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut stats_panel_query: Query<&mut Node, With<HorseStatsPanel>>,
    mut selected: ResMut<SelectedHorses>,
) {
    for (interaction, button, children, mut border, mut bg) in &mut interaction_query {
        let index = button.0;

        match *interaction {
            Interaction::Hovered => {
                // show stats panel
                for child in children.iter() {
                    if let Ok(mut node) = stats_panel_query.get_mut(child) {
                        node.display = Display::Flex;
                    }
                }

                // lightene bg slightly
                bg.0 = Color::srgb(0.25, 0.25, 0.25);
            }

            Interaction::None => {
                // hide stats panel
                for child in children.iter() {
                    if let Ok(mut node) = stats_panel_query.get_mut(child) {
                        node.display = Display::None;
                    }
                }

                // reset bg unless selected
                if selected.0.contains(&index) {
                    bg.0 = Color::srgb(0.25, 0.25, 0.25);
                } else {
                    bg.0 = Color::srgb(0.2, 0.2, 0.2);
                }
            }

            Interaction::Pressed => {
                // toggle selection
                if selected.0.contains(&index) {
                    selected.0.retain(|&i| i != index);
                } else {
                    selected.0.push(index);
                }
            }
        }

        // border update
        if selected.0.contains(&index) {
            *border = BorderColor::all(Color::srgb(0.0, 1.0, 0.0));
        } else {
            *border = BorderColor::all(Color::WHITE);
        }
    }
}

pub fn start_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<StartRaceButton>>,
    selected: Res<SelectedHorses>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let (interaction, mut colour) = interaction_query.single_mut().unwrap();
	let enabled = !selected.0.is_empty();

	if selected.0.is_empty() {
		*colour = BackgroundColor(Color::Srgba(css::GREY))
	}
	else {
		*colour = BackgroundColor(Color::Srgba(css::RED))
	}

	if enabled && *interaction == Interaction::Hovered {
		colour.0 = Color::srgb(0.9, 0.2, 0.2);
	}

	if *interaction == Interaction::Pressed && enabled {
		next_state.set(GameState::Betting);
	}
}
