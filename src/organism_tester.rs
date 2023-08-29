use std::time::Duration;

use bevy::{math::vec2, prelude::*};

use crate::organism::{
    bone::Bone,
    joint::{self, JointBundle},
    muscle::{self, Muscle},
    organism::{Organism, OrganismList},
};

#[derive(Resource)]
pub struct GenerationSpawnConfig {
    timer: Timer,
}
pub fn update_timer(
    mut commands: Commands,
    mut config: ResMut<GenerationSpawnConfig>,
    mut ol: ResMut<OrganismList>,
    time: Res<Time>,
) {
    config.timer.tick(time.delta());
    if ol.organisms.is_empty() {
        spawn_generation(&mut commands, &config);
        return;
    }

    if config.timer.finished() {
        config.timer = Timer::new(Duration::from_secs(5), TimerMode::Once);
        ol.despawn(&mut commands);
        spawn_generation(&mut commands, &config);
    }
}

pub struct OrganismTestingPlugin;
impl Plugin for OrganismTestingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenerationSpawnConfig {
            timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
        })
        .insert_resource(OrganismList::new());
        app.add_systems(
            Update,
            update_timer.run_if(resource_exists::<OrganismList>()),
        );
        // app.add_systems(Startup, spawn_generation);
        // app.add_systems(Startup, spawn_organism_test);
    }
}

fn spawn_generation(commands: &mut Commands, config: &GenerationSpawnConfig) {
    let organism = spawn_running_organism(commands);
    commands.insert_resource(OrganismList {
        organisms: vec![organism],
    });
}

fn spawn_running_organism(commands: &mut Commands) -> Organism {
    let brain_structure = vec![6, 6, 6];
    let joint_pos = vec![
        vec2(-80.0, 200.0),
        vec2(80.0, 200.0),
        vec2(-100.0, 140.0),
        vec2(0.0, 160.0),
        vec2(100.0, 140.0),
        vec2(-60.0, 100.0),
        vec2(60.0, 100.0),
        vec2(-90.0, 10.0),
        vec2(90.0, 10.0),
    ];
    let bones = vec![
        [0, 1],
        [0, 2],
        [3, 0],
        [3, 1],
        [1, 4],
        [2, 3],
        [4, 3],
        [5, 2],
        [6, 4],
        [5, 7],
        [6, 8],
    ];
    let muscles = vec![[2, 7], [3, 5], [3, 6], [4, 8]];

    // let organism = Organism::new(commands, brain_structure, joint_pos, vec![], vec![]);
    let organism = Organism::new(commands, brain_structure, joint_pos, bones, muscles);
    return organism;
}
