use bevy::{
    prelude::{
        default, Camera, Commands, Component, Entity, GlobalTransform, Query, Res, Transform, With,
        Without,
    },
    window::{PrimaryWindow, Window},
};
use bevy_rapier2d::prelude::{QueryFilter, QueryFilterFlags, RapierContext};

use crate::controls::{camera::ScrollingCam, control_state::ControlState};

use super::{
    construction_mode::{ConstructionMode, Mode},
    icons::{DraggableIcon, JointIcon},
};

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
    draggable_icons: Query<Entity, With<DraggableIcon>>,
    rapier_context: Res<RapierContext>,
    cm: Res<ConstructionMode>,
) {
    if cm.current_mode != Mode::Joint {
        return;
    }
    match cs.left_mouse_down {
        true => rapier_context.intersections_with_point(
            cs.world_mouse_pos,
            QueryFilter {
                flags: QueryFilterFlags::EXCLUDE_SOLIDS,
                ..default()
            },
            |e| match draggable_icons.get(e) {
                Ok(_) => {
                    commands.entity(e).insert(Dragging);
                    false
                }
                Err(_) => true,
            },
        ),
        false => {
            for e in draggable_icons.iter() {
                commands.entity(e).remove::<Dragging>();
            }
        }
    }
}
