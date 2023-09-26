use bevy::prelude::{
    resource_exists, App, Commands, IntoSystemConfigs, Plugin, Res, ResMut, Resource, Update,
};

use crate::{
    config::structs::{GenerationConfig, SaveConfig},
    generation::{environment::Environment, setup_builders},
    handles::Handles,
    organism::organism_list::OrganismList,
    organism_constructor::constructor::Constructor,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scene {
    NoScene,
    StartMenu,
    OrganismConstructor,
    OrganismSimulation,
}
impl Scene {
    fn pre_change(
        &self,
        commands: &mut Commands,
        con: &mut Constructor,
        ol: &mut OrganismList,
        env: &Environment,
    ) {
        match self {
            Scene::NoScene => {}
            Scene::StartMenu => {}
            Scene::OrganismConstructor => {
                con.despawn(commands);
            }
            Scene::OrganismSimulation => {
                ol.despawn(commands);
                env.despawn(commands);
            }
        }
    }
    fn post_change(
        &self,
        commands: &mut Commands,
        con: &mut Constructor,
        ol: &mut OrganismList,
        env: &mut Environment,
        handles: &Handles,
        gc: &GenerationConfig,
        sc: &SaveConfig,
    ) {
        match self {
            Scene::NoScene => {}
            Scene::StartMenu => {}
            Scene::OrganismConstructor => {
                con.spawn(commands);
            }
            Scene::OrganismSimulation => {
                setup_builders(ol, gc, sc);
                ol.spawn(commands, handles, gc.vertical_sep);
                env.spawn(commands, &handles.block_mesh, &handles.block_material, gc);
            }
        }
    }
}

#[derive(Resource)]
struct CurrentScene {
    cur_scene: Scene,
    next_scene: Scene,
}

fn scene_needs_change(cs: Res<CurrentScene>) -> bool {
    return cs.cur_scene != cs.next_scene;
}
fn change_scene(
    mut commands: Commands,
    mut cs: ResMut<CurrentScene>,
    mut con: ResMut<Constructor>,
    mut ol: ResMut<OrganismList>,
    mut env: ResMut<Environment>,
    gc: Res<GenerationConfig>,
    sc: Res<SaveConfig>,
    handles: Res<Handles>,
) {
    cs.cur_scene
        .pre_change(&mut commands, &mut con, &mut ol, &env);
    cs.cur_scene = cs.next_scene;
    cs.next_scene.post_change(
        &mut commands,
        &mut con,
        &mut ol,
        &mut env,
        &handles,
        &gc,
        &sc,
    );
}

pub struct SceneManagerPlugin;
impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentScene {
            cur_scene: Scene::NoScene,
            next_scene: Scene::OrganismConstructor,
        })
        .add_systems(Update, (change_scene).run_if(scene_needs_change));
    }
}
