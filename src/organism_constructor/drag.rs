use bevy::{
    prelude::{default, Commands, Component, Entity, Query, Res, Transform, With},
    window::{PrimaryWindow, Window},
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::controls::control_state::ControlState;

use super::icons::Icon;

#[derive(Component)]
pub struct Dragging;

pub fn drag_draggable(
    mut dragging: Query<&mut Transform, With<Dragging>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(mouse_pos) = windows.single().cursor_position() {
        if let Ok(mut t) = dragging.get_single_mut() {
            let y = t.translation.y;
            t.translation = mouse_pos.extend(y);
        }
    }
}

pub fn set_draggable(
    mut commands: Commands,
    cs: Res<ControlState>,
    rapier_context: Res<RapierContext>,
    icons: Query<Entity, With<Icon>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    match cs.left_mouse_down {
        true => {
            if let Some(mouse_pos) = windows.single().cursor_position() {
                rapier_context.intersections_with_point(
                    mouse_pos,
                    QueryFilter {
                        flags: QueryFilterFlags::EXCLUDE_SOLIDS,
                        ..default()
                    },
                    |entity| {
                        commands.entity(entity).insert(Dragging);
                        true
                    },
                )
            }
        }
        false => {
            if let Ok(e) = icons.get_single() {
                commands.entity(e).remove::<Dragging>();
            }
        }
    }
}
