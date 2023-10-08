use bevy::{
    prelude::{
        default, resource_changed, BuildChildren, Button, ButtonBundle, Changed, ChildBuilder,
        Children, Commands, Component, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource,
        TextBundle, Update, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, Style, Val},
};

use crate::color_palette;

use super::constructor::AnchoredIconConstruction;

pub struct ConstructionModePlugin;
impl Plugin for ConstructionModePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ConstructionMode::new());
        app.add_systems(Update, handle_mode_buttons);
        app.add_systems(
            Update,
            handle_mode_button_color.run_if(resource_changed::<ConstructionMode>()),
        );
    }
}

#[derive(Component)]
pub struct ModeButton {
    mode: Mode,
    pressed_text: String,
    hover_text: String,
    none_text: String,
}
impl ModeButton {
    pub fn new(
        mode: Mode,
        pressed_text: &str,
        hover_text: &str,
        none_text: &str,
        cell: &mut ChildBuilder,
    ) {
        if mode == Mode::None {
            panic!("Cannot create a button with no mode");
        }
        cell.spawn((
            ModeButton {
                mode,
                pressed_text: pressed_text.to_string(),
                hover_text: hover_text.to_string(),
                none_text: none_text.to_string(),
            },
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
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
                none_text,
                TextStyle {
                    font_size: 40.0,
                    color: color_palette::TERTIARY,
                    ..default()
                },
            ));
        });
    }

    fn handle_click(&self, text: &mut String, interaction: Interaction, cm: &mut ConstructionMode) {
        match interaction {
            Interaction::Pressed => {
                *text = self.pressed_text.clone();
                cm.current_mode = self.mode;
            }
            Interaction::Hovered => *text = self.hover_text.clone(),
            Interaction::None => *text = self.none_text.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    None,
    Joint,
    Bone,
    Muscle,
    Create,
}

#[derive(Resource)]
pub struct ConstructionMode {
    pub current_mode: Mode,
}
impl ConstructionMode {
    pub fn new() -> Self {
        return Self {
            current_mode: Mode::None,
        };
    }
}

fn handle_mode_buttons(
    buttons: Query<(&Interaction, &Children, &ModeButton), (Changed<Interaction>, With<Button>)>,
    mut button_texts: Query<&mut Text>,
    mut cm: ResMut<ConstructionMode>,
) {
    for (i, children, mb) in buttons.iter() {
        let text = &mut button_texts.get_mut(children[0]).unwrap().sections[0].value;
        mb.handle_click(text, *i, &mut cm);
    }
}

fn handle_mode_button_color(
    mut commands: Commands,
    mut buttons: Query<(&mut BackgroundColor, &mut BorderColor, &ModeButton), With<Button>>,
    cm: Res<ConstructionMode>,
    mut aic: ResMut<AnchoredIconConstruction>,
) {
    aic.clear(&mut commands);
    for (mut back_color, mut border_color, mb) in buttons.iter_mut() {
        match cm.current_mode == mb.mode {
            true => {
                back_color.0 = color_palette::SELECTED;
                border_color.0 = color_palette::SELECTED;
            }
            false => {
                back_color.0 = color_palette::NOT_SELECTED;
                border_color.0 = color_palette::NOT_SELECTED;
            }
        }
    }
}
