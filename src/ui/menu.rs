use std::thread::spawn;

use crate::{GameState, components::*};
use bevy::{ecs::spawn, prelude::*};

fn setup_track_select(mut commands: Commands, asset_server: Res<AssetServer>) {
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
			parent.spawn((
				Node {
					flex_direction: FlexDirection::Row,
					column_gap: Val::Px(20.0),
					..default()
				}
			));
        });
}
