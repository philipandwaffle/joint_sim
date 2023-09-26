use bevy::{
    prelude::{default, BuildChildren, Bundle, Color, Commands, Entity, NodeBundle},
    ui::{BackgroundColor, Display, GridTrack, Style, UiRect, Val},
};

use crate::color_palette;

#[derive(Bundle)]
pub struct PartMenuBundle {
    node_bundle: NodeBundle,
}
impl PartMenuBundle {
    pub fn new(commands: &mut Commands) -> Entity {
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
                        grid_template_columns: vec![GridTrack::auto(); 3],
                        ..default()
                    },
                    background_color: BackgroundColor(color_palette::SECONDARY),
                    ..default()
                },
            })
            .with_children(|grid| {
                grid.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        margin: UiRect {
                            left: Val::Percent(2.5),
                            right: Val::Percent(2.5),
                            top: Val::Percent(0.25),
                            bottom: Val::Percent(0.25),
                        },
                        ..default()
                    },
                    background_color: BackgroundColor(color_palette::PRIMARY),
                    ..default()
                });
                grid.spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        margin: UiRect {
                            left: Val::Percent(2.5),
                            right: Val::Percent(2.5),
                            top: Val::Percent(2.5),
                            bottom: Val::Percent(2.5),
                        },
                        ..default()
                    },
                    background_color: BackgroundColor(color_palette::PRIMARY),
                    ..default()
                });
                grid.spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        margin: UiRect {
                            left: Val::Percent(2.5),
                            right: Val::Percent(2.5),
                            top: Val::Percent(0.25),
                            bottom: Val::Percent(0.25),
                        },
                        ..default()
                    },
                    background_color: BackgroundColor(color_palette::PRIMARY),
                    ..default()
                });
            })
            .id();

        return grid_ent;
    }
}
