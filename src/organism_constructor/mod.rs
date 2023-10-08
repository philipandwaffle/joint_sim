use bevy::prelude::{
    IntoSystemConfigs, Parent, Plugin, Query, Res, ResMut, Transform, Update, With,
};

use crate::{
    config::structs::GenerationConfig,
    organism::organism_list::OrganismList,
    scene_manager::{CurrentScene, Scene},
};

use self::{
    construction_mode::{ConstructionMode, ConstructionModePlugin, Mode},
    constructor::{
        handle_anchored_icon_construction, handle_joint_construction, AnchoredIconConstruction,
        Constructor,
    },
    drag::{move_dragging, set_draggable},
    icons::{anchor_icons, AnchorPoint, AnchorSet, BoneIcon, JointIcon, MuscleIcon},
};

mod construction_grid;
mod construction_mode;
pub mod constructor;
mod drag;
mod icons;
pub mod mode_menu;

pub struct OrganismConstructionPlugin;
impl Plugin for OrganismConstructionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Constructor::default())
            .insert_resource(AnchoredIconConstruction::default())
            .add_plugins(ConstructionModePlugin)
            .add_systems(Update, (anchor_icons, move_dragging, set_draggable))
            .add_systems(Update, handle_joint_construction.run_if(construct_joint))
            .add_systems(
                Update,
                handle_anchored_icon_construction.run_if(construct_anchored_icon),
            )
            .add_systems(Update, construct.run_if(construct_organism));
    }
}

fn construct_joint(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Joint;
}
fn construct_anchored_icon(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Bone || cm.current_mode == Mode::Muscle;
}

fn construct_organism(cm: Res<ConstructionMode>) -> bool {
    return cm.current_mode == Mode::Create;
}

fn construct(
    c: Res<Constructor>,
    gc: Res<GenerationConfig>,
    mut ol: ResMut<OrganismList>,
    mut cs: ResMut<CurrentScene>,
    mut cm: ResMut<ConstructionMode>,
    joint_icons: Query<(&Transform, &JointIcon)>,
    anchors: Query<&Parent, With<AnchorPoint>>,
    bone_anchors: Query<(&AnchorSet, &BoneIcon)>,
    muscle_anchors: Query<(&AnchorSet, &MuscleIcon)>,
) {
    cm.current_mode = Mode::None;
    match c.create_builder(&joint_icons, &anchors, &bone_anchors, &muscle_anchors) {
        Ok(ob) => {
            ol.builders = vec![ob; gc.num_organisms];
            cs.next_scene = Scene::OrganismSimulation;
        }
        Err(e) => println!("Couldn't create organism {:?}", e),
    };
}
