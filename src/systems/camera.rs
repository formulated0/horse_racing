use crate::components::*;
use bevy::prelude::*;

pub fn camera_switching(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &HorseNumber)>,
) {
    let mut target_num: Option<usize> = None;
    if keys.just_pressed(KeyCode::Digit1) {
        target_num = Some(0);
    } else if keys.just_pressed(KeyCode::Digit2) {
        target_num = Some(1);
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
