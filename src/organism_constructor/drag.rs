use bevy::{
    prelude::{
        default, Camera, Commands, Component, Entity, GlobalTransform, Query, Res, Transform, With,
    },
    window::{PrimaryWindow, Window},
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::controls::{camera::ScrollingCam, control_state::ControlState};

use super::icons::Icon;

#[derive(Component)]
pub struct Dragging;

pub fn move_dragging(mut dragging: Query<&mut Transform, With<Dragging>>, cs: Res<ControlState>) {
    if let Ok(mut t) = dragging.get_single_mut() {
        let z = t.translation.z;
        t.translation = cs.world_mouse_pos.extend(z);
    }
}

pub fn set_draggable(
    mut commands: Commands,
    cs: Res<ControlState>,
    rapier_context: Res<RapierContext>,
    icons: Query<Entity, With<Icon>>,
) {
    match cs.left_mouse_down {
        true => rapier_context.intersections_with_point(
            cs.world_mouse_pos,
            QueryFilter {
                flags: QueryFilterFlags::EXCLUDE_SOLIDS,
                ..default()
            },
            |e| {
                commands.entity(e).insert(Dragging);
                true
            },
        ),
        false => {
            if let Ok(e) = icons.get_single() {
                // println!("remove dragging");
                commands.entity(e).remove::<Dragging>();
            }
        }
    }
}
