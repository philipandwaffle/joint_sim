use bevy::{
    math::vec2,
    prelude::{default, BuildChildren, Bundle, Commands, Entity, NodeBundle},
    ui::{BackgroundColor, Display, GridTrack, Style, Val},
};

use crate::{color_palette, handles::Handles};

use super::icons::JointIcon;

#[derive(Bundle)]
pub struct PartMenuBundle {
    node_bundle: NodeBundle,
}
impl PartMenuBundle {
    pub fn new(commands: &mut Commands, handles: &Handles) -> Entity {
        let grid_ent = commands
            .spawn(Self {
                node_bundle: NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        height: Val::Percent(20.0),
                        width: Val::Percent(100.0),
                        grid_template_rows: vec![GridTrack::auto(); 1],
                        grid_template_columns: vec![GridTrack::percent(33.3); 3],
                        ..default()
                    },
                    background_color: BackgroundColor(color_palette::SECONDARY),
                    ..default()
                },
            })
            .with_children(|grid| {
                for i in 0..3 {
                    grid.spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            left: Val::Percent(2.0),
                            top: Val::Percent(10.0),
                            width: Val::Percent(96.0),
                            height: Val::Percent(80.0),
                            ..default()
                        },
                        background_color: BackgroundColor(color_palette::PRIMARY),
                        ..default()
                    })
                    .with_children(|cell| {
                        cell.spawn(JointIcon::new(
                            vec2(0.0, 0.0),
                            5.0,
                            &handles.joint_mesh,
                            &handles.joint_material,
                        ));
                    });
                }
            })
            .id();

        return grid_ent;
    }
}
