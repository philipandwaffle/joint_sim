use bevy::{
    prelude::{default, BuildChildren, Bundle, ChildBuilder, Color, Commands, Entity, NodeBundle},
    ui::{BackgroundColor, Display, GridTrack, Style, UiRect, Val},
};

pub struct ConstructionGridConfig {
    cell_size: f32,
    snap: bool,
}

#[derive(Bundle)]
pub struct ConstructionGridBundle {
    node_bundle: NodeBundle,
}
impl ConstructionGridBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        let num_rows = 10;
        let num_cols = 10;
        let grid_ent = commands
            .spawn(Self {
                node_bundle: NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        left: Val::Percent(20.0),
                        top: Val::Percent(20.0),
                        height: Val::Px(500.0),
                        width: Val::Px(500.0),
                        grid_template_rows: vec![GridTrack::auto(); num_rows],
                        grid_template_columns: vec![GridTrack::auto(); num_cols],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hsla(0.0, 0.25, 0.25, 0.3)),
                    ..default()
                },
            })
            .with_children(|p| {
                for _ in 0..(num_rows * num_rows) {
                    push_cell(p);
                }
            })
            .id();

        return grid_ent;
    }
}
fn push_cell(grid: &mut ChildBuilder) {
    grid.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,
            padding: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    })
    .with_children(|builder| {
        builder.spawn(NodeBundle {
            // background_color: BackgroundColor(Color::hsla(240.0, 30.0, 50.0, 20.0)),
            ..default()
        });
    });
}
