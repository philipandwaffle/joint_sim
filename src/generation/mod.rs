use bevy::{
    math::vec2,
    prelude::{
        default, resource_exists, App, Commands, IntoSystemConfigs, Plugin, Query, Res, ResMut,
        Startup, Transform, Update, Vec2, With,
    },
    time::{Time, Timer, TimerMode},
};
use rand::Rng;
use std::time::{Duration, Instant};

use self::{config::GenerationConfig, environment::spawn_environment};
use crate::organism::{joint::Joint, organism::OrganismBuilder, organism_list::OrganismList};

pub mod config;
mod environment;

pub struct GenerationPlugin;
impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GenerationConfig {
            num_organisms: 500,
            vertical_sep: 200.0,
            timer: Timer::new(Duration::from_secs(20), TimerMode::Once),
            unfreeze_flag: true,
            debug_flag: false,
        })
        .insert_resource(OrganismList::new())
        .add_systems(Startup, (spawn_environment, setup_organism_list))
        .add_systems(
            Update,
            handle_generation.run_if(resource_exists::<OrganismList>()),
        );
    }
}

pub fn handle_generation(
    mut commands: Commands,
    mut config: ResMut<GenerationConfig>,
    mut ol: ResMut<OrganismList>,
    time: Res<Time>,
    joint_transforms: Query<&Transform, With<Joint>>,
) {
    config.timer.tick(time.delta());
    if ol.builders.is_empty() {
        return;
    } else if ol.organisms.is_empty() {
        ol.spawn(&mut commands, config.vertical_sep);
        return;
    }

    let elapsed_secs = config.timer.elapsed_secs();
    if config.unfreeze_flag && elapsed_secs > 0.5 {
        ol.unfreeze();
        config.unfreeze_flag = false;
    }
    if config.debug_flag && (elapsed_secs % 0.5) <= 0.05 {
        println!("{:?}", ol.organisms[0].brain.memory);
    }

    if config.timer.finished() {
        config.timer.reset();
        config.timer.unpause();
        config.unfreeze_flag = true;

        let new_builders = get_next_generation_builders(&mut ol, &mut config, &joint_transforms);

        // Spawn new generation
        ol.despawn(&mut commands);
        ol.builders = new_builders;
        ol.spawn(&mut commands, config.vertical_sep);
    }
}

fn get_next_generation_builders(
    ol: &mut OrganismList,
    config: &mut GenerationConfig,
    joint_transforms: &Query<&Transform, With<Joint>>,
) -> Vec<OrganismBuilder> {
    // Calculate fitness
    let num_organism = config.num_organisms;
    let mut fitness = Vec::with_capacity(num_organism);
    for o in ol.organisms.iter() {
        let score = o
            .joints
            .iter()
            .map(|x| joint_transforms.get(*x).unwrap().translation.x)
            .sum::<f32>()
            / o.joints.len() as f32;
        if score.is_nan() {
            fitness.push(0.0);
        } else {
            fitness.push(score)
        };
    }

    // Pick the 'best' organisms
    let fitness_unsorted = fitness.clone();
    fitness.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let avg_fitness = fitness.iter().sum::<f32>() / fitness.len() as f32;
    let median_fitness = fitness[fitness.len() / 2];
    let upper_10 = fitness[(fitness.len() as f32 * 0.9) as usize];
    // println!(
    //     "average fitness {:?}\t median fitness {:?}\t upper_10 {:?}",
    //     avg_fitness, median_fitness, upper_10
    // );
    println!("{:?},{:?},{:?}", avg_fitness, median_fitness, upper_10);
    let mut new_builders = Vec::with_capacity(num_organism);
    for i in 0..num_organism {
        if fitness_unsorted[i] >= median_fitness {
            new_builders.push(ol.builders[i].clone());
        }
    }

    // Clone random organisms to fill the vec
    // println!("num builders {}", new_builders.len());
    let mut rng = rand::thread_rng();
    while new_builders.len() < num_organism {
        let index = rng.gen_range(0..new_builders.len());
        let mut new_builder = new_builders[index].clone();
        new_builders.push(new_builder);
    }

    // Mutate each organism
    new_builders.iter_mut().for_each(|x| x.mutate());

    return new_builders;
}

fn setup_organism_list(mut commands: Commands, config: Res<GenerationConfig>) {
    let mut builders = vec![];
    for _ in 0..config.num_organisms {
        builders.push(get_runner_builder());
    }
    let ol = OrganismList {
        builders: builders,
        organisms: vec![],
        is_spawned: false,
    };

    commands.insert_resource(ol);
}

fn get_simple_builder() -> OrganismBuilder {
    let brain_structure = vec![6, 6];
    let joint_pos = vec![
        vec2(-20.0, 60.0),
        vec2(20.0, 60.0),
        vec2(-70.0, 40.0),
        vec2(0.0, 40.0),
        vec2(70.0, 40.0),
        vec2(-60.0, 5.0),
        vec2(-20.0, 5.0),
        vec2(20.0, 5.0),
        vec2(60.0, 5.0),
    ];
    let bones = vec![
        [0, 1],
        [2, 0],
        [0, 3],
        [1, 3],
        [4, 1],
        [3, 2],
        [2, 4],
        [5, 0],
        [6, 3],
        [7, 3],
        [8, 1],
    ];
    let muscles = vec![[5, 2], [6, 3], [7, 3], [8, 4]];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

fn get_runner_builder() -> OrganismBuilder {
    let brain_structure = vec![3, 3];
    let joint_pos = vec![
        vec2(-20.0, 80.0),
        vec2(20.0, 80.0),
        vec2(-70.0, 60.0),
        vec2(0.0, 60.0),
        vec2(70.0, 60.0),
        vec2(-40.0, 25.0),
        vec2(40.0, 25.0),
    ];
    // let joint_pos = vec![
    //     vec2(-20.0, 60.0),
    //     vec2(20.0, 60.0),
    //     vec2(-70.0, 40.0),
    //     vec2(0.0, 40.0),
    //     vec2(70.0, 40.0),
    //     vec2(-40.0, 5.0),
    //     vec2(40.0, 5.0),
    // ];

    let bones = vec![
        [0, 1],
        [2, 0],
        [0, 3],
        [1, 3],
        [4, 1],
        [5, 0],
        [6, 1],
        [3, 2],
        [3, 4],
    ];
    let muscles = vec![
        [5, 2],
        // [5, 3],
        // [6, 3],
        [6, 4],
    ];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}
