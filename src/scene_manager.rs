use bevy::prelude::{App, Commands, IntoSystemConfigs, Plugin, Res, ResMut, Resource, Update};

use crate::{
    config::structs::GenerationConfig, generation::environment::Environment, handles::Handles,
    organism::organism_list::OrganismList,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scene {
    StartMenu,
    OrganismConstructor,
    OrganismSimulation,
}
impl Scene {
    fn pre_change(&self, commands: &mut Commands, ol: &mut OrganismList, env: &Environment) {
        match self {
            Scene::StartMenu => {}
            Scene::OrganismConstructor => {}
            Scene::OrganismSimulation => {
                ol.despawn(commands);
                env.despawn(commands);
            }
        }
    }
    fn post_change(
        &self,
        commands: &mut Commands,
        ol: &mut OrganismList,
        env: &mut Environment,
        handles: &Handles,
        gc: &GenerationConfig,
    ) {
        match self {
            Scene::StartMenu => {}
            Scene::OrganismConstructor => {}
            Scene::OrganismSimulation => {
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
    return cs.cur_scene == cs.next_scene;
}
fn change_scene(
    mut commands: Commands,
    mut cs: ResMut<CurrentScene>,
    mut ol: ResMut<OrganismList>,
    mut env: ResMut<Environment>,
    handles: Res<Handles>,
    gc: Res<GenerationConfig>,
) {
    cs.cur_scene.pre_change(&mut commands, &mut ol, &env);
    cs.cur_scene = cs.next_scene;
    cs.next_scene
        .post_change(&mut commands, &mut ol, &mut env, &handles, &gc);
}

pub struct SceneManagerPlugin;
impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentScene {
            cur_scene: Scene::StartMenu,
            next_scene: Scene::StartMenu,
        })
        .add_systems(Update, (change_scene).run_if(scene_needs_change));
    }
}
