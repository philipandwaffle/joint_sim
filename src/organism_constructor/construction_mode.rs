use bevy::{
    prelude::{
        default, BuildChildren, Button, ButtonBundle, Changed, ChildBuilder, Children, Color,
        Component, Plugin, Query, Resource, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, Style, Val},
};

use crate::color_palette;

pub struct ConstructionModePlugin;
impl Plugin for ConstructionModePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ConstructionMode::new());
    }
}

#[derive(Component)]
pub struct ModeButton {
    mode: Mode,
}
impl ModeButton {
    pub fn new(mode: Mode, cell: &mut ChildBuilder) {
        if mode == Mode::None {
            panic!("Cannot create a button with no mode");
        }
        cell.spawn((
            ModeButton { mode },
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(color_palette::TERTIARY),
                background_color: BackgroundColor(color_palette::PRIMARY),
                ..default()
            },
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 40.0,
                    color: color_palette::TERTIARY,
                    ..default()
                },
            ));
        });
    }
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    None,
    Joint,
    Bone,
    Muscle,
}

#[derive(Resource)]
pub struct ConstructionMode {
    current_mode: Mode,
    next_mode: Mode,
}
impl ConstructionMode {
    pub fn new() -> Self {
        return Self {
            current_mode: Mode::None,
            next_mode: Mode::Joint,
        };
    }
}

fn change_construction_mode(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
}
