use bevy::prelude::{App, Commands, IntoSystemConfigs, Plugin, Res, ResMut, Resource, Update};

use crate::{
    config::structs::{GenerationConfig, SaveConfig},
    generation::environment::Environment,
    handles::Handles,
    organism::organism_list::OrganismList,
    organism_constructor::constructor::Constructor,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scene {
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
        gc: &mut GenerationConfig,
        handles: &Handles,
        sc: &SaveConfig,
    ) {
        match self {
            Scene::NoScene => {}
            Scene::StartMenu => {}
            Scene::OrganismConstructor => {
                con.spawn(commands);
            }
            Scene::OrganismSimulation => {
                // setup_builders(ol, gc, sc);
                gc.timer.reset();
                ol.spawn(commands, handles, gc.vertical_sep);
                env.spawn(commands, &handles.block_mesh, &handles.block_material, gc);
            }
        }
    }
}

#[derive(Resource)]
pub struct CurrentScene {
    cur_scene: Scene,
    pub next_scene: Scene,
}
pub fn is_simulation(cs: Res<CurrentScene>) -> bool {
    return cs.cur_scene == Scene::OrganismSimulation;
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
    mut gc: ResMut<GenerationConfig>,
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
        &mut gc,
        &handles,
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
