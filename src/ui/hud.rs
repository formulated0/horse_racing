use crate::components::*;
use bevy::prelude::*;
use bevy::text::{TextWriter, TextReader};


#[derive(Component)]
pub struct DebugText {}

pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
		.spawn((
			Text::new("Debug"),
			TextFont {
				font_size: 30.0,
				..default()
			},
			TextColor(Color::WHITE),
			Node {
				position_type: PositionType::Absolute,
				top: Val::Px(10.0),
				left: Val::Px(10.0),
				..default()
			},
    ))
	.insert(DebugText{});
}

pub fn update_hud(
    horse_q: Query<(&RaceState, &BaseStats, &HorseName), With<PlayerFocus>>,
    text_q: Query<Entity, With<DebugText>>,
    mut text_writer: TextWriter<Text>,
) {
    let Ok((state, stats, name)) = horse_q.single() else { return };
    let Ok(text_entity) = text_q.single() else { return };

    *text_writer.text(text_entity, 0) = format!(
        "Name: {}\nSpeed: {:.1} m/s\nStamina: {:.1}/{:.1}\nDist: {:.1}m",
		name.0,
        state.current_speed,
        state.current_stamina,
        stats.stamina,
        state.distance_traveled
    );
}