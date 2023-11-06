use bevy::{
    prelude::{default, BuildChildren, Bundle, Commands, Component, Entity, NodeBundle},
    ui::{BackgroundColor, Display, GridTrack, PositionType, Style, Val},
};

use crate::color_palette;

use super::construction_mode::{Mode, ModeButton};

#[derive(Component)]
pub struct Menu;
#[derive(Bundle)]
pub struct ModeMenuBundle {
    node_bundle: NodeBundle,
    menu: Menu,
}
impl ModeMenuBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        let grid_ent = commands
            .spawn(Self {
                node_bundle: NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        position_type: PositionType::Absolute,
                        left: Val::Percent(0.0),
                        top: Val::Percent(0.0),
                        height: Val::Percent(20.0),
                        width: Val::Percent(100.0),
                        grid_template_rows: vec![GridTrack::auto(); 1],
                        grid_template_columns: vec![GridTrack::percent(25.0); 4],
                        ..default()
                    },
                    background_color: BackgroundColor(color_palette::SECONDARY),
                    ..default()
                },
                menu: Menu,
            })
            .with_children(|grid| {
                for i in 0..4 {
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
                        if i == 0 {
                            ModeButton::new(
                                Mode::Joint,
                                "Joint",
                                "Double click to create a joint",
                                "Joint",
                                cell,
                            );
                        } else if i == 1 {
                            ModeButton::new(
                                Mode::Bone,
                                "Bone",
                                "Click and drag between two joints to create a bone",
                                "Bone",
                                cell,
                            );
                        } else if i == 2 {
                            ModeButton::new(
                                Mode::Muscle,
                                "Muscle",
                                "Click and drag between two bones to create a muscle",
                                "Muscle",
                                cell,
                            );
                        } else if i == 3 {
                            ModeButton::new(
                                Mode::Create,
                                "Create Organism",
                                "Click to complete construction",
                                "Create Organism",
                                cell,
                            );
                        }
                    });
                }
            })
            .id();

        return grid_ent;
    }
}
