use bevy::{
    prelude::{default, BuildChildren, Color, Commands, NodeBundle},
    ui::{PositionType, Style, Val},
};

use super::construction_grid::ConstructionGridBundle;

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
