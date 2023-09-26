use bevy::{
    prelude::{default, BuildChildren, Bundle, Color, Commands, Entity, NodeBundle},
    ui::{BackgroundColor, Display, GridTrack, PositionType, Style, Val},
};

use super::construction_grid::ConstructionGridBundle;
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
                        height: Val::Percent(10.0),
                        width: Val::Percent(100.0),
                        grid_template_rows: vec![GridTrack::auto(); 1],
                        grid_template_columns: vec![GridTrack::auto(); 3],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hsla(0.0, 0.25, 0.25, 0.3)),
                    ..default()
                },
            })
            .with_children(|p| {
                
            })
            .id();

        return grid_ent;
    }
}

pub fn ui_test(mut commands: Commands) {
    ConstructionGridBundle::new(&mut commands);

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                ..default()
            },
            background_color: Color::ANTIQUE_WHITE.into(),
            ..default()
        })
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Percent(50.0),
                    height: Val::Percent(50.0),
                    left: Val::Percent(0.0),
                    top: Val::Percent(0.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            });
        });
}
