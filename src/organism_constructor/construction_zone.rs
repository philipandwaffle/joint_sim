use bevy::{
    prelude::{
        default, Bundle, Changed, Color, Commands, Component, Entity, NodeBundle, Query, ResMut,
        With,
    },
    ui::{BackgroundColor, Display, Interaction, PositionType, Style, Val},
};

use super::constructor::Constructor;

#[derive(Component)]
pub struct Zone;

#[derive(Bundle)]
pub struct ConstructionZone {
    zone: Zone,
    node_bundle: NodeBundle,
    interaction: Interaction,
}
impl ConstructionZone {
    pub fn new(commands: &mut Commands) -> Entity {
        return commands
            .spawn(Self {
                zone: Zone,
                node_bundle: NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        position_type: PositionType::Absolute,
                        left: Val::Percent(0.0),
                        top: Val::Percent(20.0),
                        height: Val::Percent(80.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                interaction: Interaction::default(),
            })
            .id();
    }
}

pub fn in_construction_zone(
    mut c: ResMut<Constructor>,
    construction_zone: Query<&Interaction, (Changed<Interaction>, With<Zone>)>,
) {
    for i in construction_zone.iter() {
        println!("{:?}", i);
        match i {
            Interaction::Pressed => c.in_bounds = true,
            Interaction::Hovered => c.in_bounds = true,
            Interaction::None => c.in_bounds = false,
        };
    }
}
