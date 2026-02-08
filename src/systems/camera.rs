use crate::components::*;
use bevy::prelude::*;

pub fn camera_switching(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &HorseNumber)>,
) {
    let mut target_num: Option<usize> = None;
    for i in 0..10 {
        let key = match i {
            0 => KeyCode::Digit1,
            1 => KeyCode::Digit2,
            2 => KeyCode::Digit3,
            3 => KeyCode::Digit4,
            4 => KeyCode::Digit5,
            5 => KeyCode::Digit6,
            6 => KeyCode::Digit7,
            7 => KeyCode::Digit8,
            8 => KeyCode::Digit9,
            9 => KeyCode::Digit0,
            _ => continue,
        };
        if keys.just_pressed(key) {
            target_num = Some(i);
            break;
        }
    }

    let Some(target) = target_num else {
        return;
    };

    for (entity, horse_number) in query.iter() {
        if horse_number.0 == target {
            commands.entity(entity).insert(PlayerFocus {});
        } else {
            commands.entity(entity).remove::<PlayerFocus>();
        }
    }
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&Transform, (With<PlayerFocus>, Without<Camera>)>,
) {
    let Ok(mut cam_trans) = camera_query.single_mut() else {
        return;
    };

    if let Ok(target_trans) = target_query.single() {
        cam_trans.translation = cam_trans.translation.lerp(target_trans.translation, 0.05);
        cam_trans.translation.z = 999.0;
    }
}
