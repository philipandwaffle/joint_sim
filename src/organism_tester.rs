use std::time::Duration;

use bevy::{ecs::query, math::vec2, prelude::*};
use bevy_rapier2d::prelude::Damping;
use rand::Rng;

use crate::organism::{
    bone::Bone,
    joint::{self, Joint, JointBundle},
    muscle::{self, Muscle},
    organism::{Organism, OrganismList},
};

#[derive(Resource)]
pub struct GenerationConfig {
    mutate_rate: f32,
    mutate_factor: f32,
    num_organisms: usize,
    timer: Timer,
    unfreeze_flag: bool,
}
pub fn handle_generation(
    mut commands: Commands,
    mut config: ResMut<GenerationConfig>,
    mut ol: ResMut<OrganismList>,
    time: Res<Time>,
    joint_transforms: Query<&Transform, With<Joint>>,
) {
    config.timer.tick(time.delta());
    if ol.organisms.is_empty() {
        spawn_generation(&mut commands, &config);
        return;
    }

    if config.unfreeze_flag && config.timer.elapsed_secs() > 0.2 {
        ol.toggle_freeze();
        config.unfreeze_flag = false;
    }

    if config.timer.finished() {
        config.timer = Timer::new(Duration::from_secs(5), TimerMode::Once);
        config.unfreeze_flag = true;

        // fitness eval
        let num_organism = config.num_organisms;
        let mut fitness = Vec::with_capacity(num_organism);
        for o in ol.organisms.iter() {
            let mut score = 0.0;
            for j in o.joints.iter() {
                score += joint_transforms.get(*j).unwrap().translation.x;
            }
            fitness.push(score);
        }

        let avg_fitness = fitness.iter().sum::<f32>() / fitness.len() as f32;
        println!("average fitness {:?}", avg_fitness);
        let mut new_organisms = Vec::with_capacity(num_organism);
        for i in 0..num_organism {
            if fitness[i] > avg_fitness {
                new_organisms.push(ol.organisms[i].clone());
            }
        }
        let mut rng = rand::thread_rng();
        while new_organisms.len() < num_organism {
            let index = rng.gen_range(0..new_organisms.len());
            new_organisms.push(new_organisms[index].clone());
        }
        new_organisms
            .iter_mut()
            .for_each(|x| x.brain.mutate(config.mutate_rate, config.mutate_factor));

        ol.despawn(&mut commands);
        ol.organisms = new_organisms;
        spawn_generation(&mut commands, &config);
    }
}

pub struct OrganismTestingPlugin;
impl Plugin for OrganismTestingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenerationConfig {
            mutate_rate: 0.1,
            mutate_factor: 0.2,
            num_organisms: 10,
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
            unfreeze_flag: true,
        })
        .insert_resource(OrganismList::new());
        app.add_systems(
            Update,
            handle_generation.run_if(resource_exists::<OrganismList>()),
        );
        // app.add_systems(Startup, spawn_generation);
        // app.add_systems(Startup, spawn_organism_test);
    }
}

fn spawn_generation(commands: &mut Commands, config: &GenerationConfig) {
    let mut organisms = vec![];
    for i in 0..config.num_organisms {
        organisms.push(spawn_runner2(commands, vec2(0.0, 200.0 * i as f32)))
    }
    let ol = OrganismList {
        organisms: organisms,
    };

    commands.insert_resource(ol);
}

fn spawn_runner2(commands: &mut Commands, offset: Vec2) -> Organism {
    let brain_structure = vec![6, 6, 6];
    let joint_pos = vec![
        vec2(-80.0, 100.0),
        vec2(0.0, 120.0),
        vec2(80.0, 100.0),
        vec2(0.0, 80.0),
        vec2(-90.0, 10.0),
        vec2(90.0, 10.0),
    ];
    let bones = vec![[0, 1], [0, 3], [2, 1], [2, 3], [1, 3], [4, 0], [5, 2]];
    let muscles = vec![[0, 4], [2, 5]];

    let o = Organism::new(commands, offset, brain_structure, joint_pos, bones, muscles);
    return o;
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
        [2, 5],
        [4, 6],
        [5, 7],
        [6, 8],
    ];
    let muscles = vec![[2, 7], [3, 5], [3, 6], [4, 8]];

    // let organism = Organism::new(commands, brain_structure, joint_pos, vec![], vec![]);
    let organism = Organism::new(
        commands,
        Vec2::ZERO,
        brain_structure,
        joint_pos,
        bones,
        muscles,
    );
    return organism;
}
