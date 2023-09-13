use bevy::{
    math::vec2,
    prelude::{
        resource_exists, App, Commands, IntoSystemConfigs, Plugin, Query, Res, ResMut, Startup,
        Transform, Update, With,
    },
    time::{Time, Timer, TimerMode},
};
use rand::distributions::{Distribution, Uniform};
use std::{fs::File, io::BufReader, time::Duration};

use self::environment::spawn_environment;
use crate::{
    config::structs::{GenerationConfig, SaveConfig},
    controls::control_state::ControlState,
    organism::{
        brain, joint::Joint, muscle, organism::OrganismBuilder, organism_list::OrganismList,
    },
};

mod environment;

pub struct GenerationPlugin;
impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrganismList::new())
            .add_systems(Startup, (spawn_environment, setup_organism_list))
            .add_systems(
                Update,
                handle_generation.run_if(resource_exists::<OrganismList>()),
            );
    }
}

pub fn handle_generation(
    mut commands: Commands,
    mut gc: ResMut<GenerationConfig>,
    sc: Res<SaveConfig>,
    mut ol: ResMut<OrganismList>,
    time: Res<Time>,
    joint_transforms: Query<&Transform, With<Joint>>,
    mut cs: ResMut<ControlState>,
) {
    gc.timer.tick(time.delta());
    if ol.builders.is_empty() {
        return;
    } else if ol.organisms.is_empty() {
        ol.spawn(&mut commands, gc.vertical_sep);
        return;
    }

    let elapsed_secs = gc.timer.elapsed_secs();
    if gc.unfreeze_flag && elapsed_secs > 0.1 {
        ol.unfreeze();
        gc.unfreeze_flag = false;
    }
    if gc.debug_flag && (elapsed_secs % 0.5) <= 0.05 {
        println!("{:?}", ol.organisms[0].brain.memory);
    }

    if gc.timer.finished() {
        gc.timer.reset();
        gc.timer.unpause();
        gc.unfreeze_flag = true;

        if gc.cur_generation % sc.rate == 0 {
            cs.save = true;
        }
        let new_builders = get_next_generation_builders(&mut ol, &mut gc, &joint_transforms);

        // Spawn new generation
        ol.despawn(&mut commands);
        ol.builders = new_builders;
        ol.spawn(&mut commands, gc.vertical_sep);
    }
}

fn get_next_generation_builders(
    ol: &mut OrganismList,
    gc: &mut GenerationConfig,
    joint_transforms: &Query<&Transform, With<Joint>>,
) -> Vec<OrganismBuilder> {
    // Calculate fitness
    let num_organism = gc.num_organisms;
    let mut fitness = Vec::with_capacity(num_organism);
    for o in ol.organisms.iter() {
        let score = o
            .joints
            .iter()
            .map(|x| joint_transforms.get(*x).unwrap().translation.x)
            .sum::<f32>()
            / o.joints.len() as f32;

        fitness.push(score);
    }

    // Pick the 'best' organisms
    let fitness_unsorted = fitness.clone();
    fitness.sort_by(|a, b| match a.partial_cmp(b) {
        Some(res) => res,
        None => std::cmp::Ordering::Less,
    });
    let median_fitness = fitness[fitness.len() / 2];
    let mut new_builders = Vec::with_capacity(num_organism);
    for i in 0..num_organism {
        if fitness_unsorted[i] >= median_fitness {
            new_builders.push(ol.builders[i].clone());
        }
    }

    let mut rng = rand::thread_rng();
    let sample = Uniform::from(0..new_builders.len());
    while new_builders.len() < num_organism {
        let index = sample.sample(&mut rng);
        let new_builder = new_builders[index].clone();
        new_builders.push(new_builder);
    }

    // Mutate each organism
    let mut rng = rand::thread_rng();
    new_builders.iter_mut().for_each(|x| x.mutate(&mut rng));

    gc.cur_generation += 1;
    return new_builders;
}

fn setup_organism_list(mut commands: Commands, gc: Res<GenerationConfig>, sc: Res<SaveConfig>) {
    let num_organisms = gc.num_organisms;
    let mut builders;

    if sc.load_save {
        let file = match File::open(&sc.load_file) {
            Ok(f) => f,
            Err(err) => {
                println!("Error loading {:?}, {:?}", sc.load_file, err);
                return;
            }
        };
        let reader = BufReader::new(file);
        builders = match serde_json::from_reader(reader) {
            Ok(json) => json,
            Err(err) => {
                println!("Error converting json {:?}", err);
                return;
            }
        };
    } else {
        builders = Vec::with_capacity(num_organisms);
        for _ in 0..num_organisms {
            builders.push(get_runner_v2());
        }
    }

    let ol = OrganismList {
        builders: builders,
        organisms: vec![],
        is_spawned: false,
    };

    commands.insert_resource(ol);
}

fn get_runner_v2() -> OrganismBuilder {
    let brain_structure = vec![10, 10, 10];
    let joint_pos = vec![
        vec2(0.0, 65.0),
        vec2(-45.0, 40.0),
        vec2(45.0, 40.0),
        vec2(-45.0, 0.0),
        vec2(-15.0, 10.0),
        vec2(15.0, 10.0),
        vec2(45.0, 0.0),
    ];

    let bones = vec![[1, 0], [0, 2], [2, 1], [3, 1], [4, 0], [5, 0], [6, 2]];
    let muscles = vec![[3, 2], [4, 0], [5, 1], [6, 2]];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

fn muscle_test_organism() -> OrganismBuilder {
    let joint_pos = vec![vec2(0.0, 0.0), vec2(25.0, 50.0), vec2(50.0, 0.0)];
    let bones = vec![[1, 2], [0, 1]];
    let muscles = vec![[1, 0]];

    return OrganismBuilder::new(1, vec![3, 3], joint_pos, bones, muscles);
}

fn bone_test_organism() -> OrganismBuilder {
    let brain_structure = vec![2, 2];

    let dx = 40.0;
    let dy0 = 40.0;
    let dy1 = 80.0;
    let dy2 = 120.0;
    let dy3 = 160.0;
    let joint_pos = vec![
        vec2(-dx, dy0),
        vec2(dx, dy0),
        vec2(0.0, dy1),
        vec2(0.0, dy2),
        vec2(-dx, dy3),
        vec2(dx, dy3),
    ];
    // let bones = vec![[0, 1], [1, 2], [2, 0]];
    let bones = vec![[0, 1], [2, 0], [2, 1], [2, 3], [3, 4], [4, 5], [5, 3]];
    let muscles = vec![];
    let ob = OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
    return ob;
}

fn get_runner_builder() -> OrganismBuilder {
    let brain_structure = vec![6, 6];
    let joint_pos = vec![
        vec2(-20.0, 80.0),
        vec2(20.0, 80.0),
        vec2(-40.0, 60.0),
        vec2(0.0, 60.0),
        vec2(40.0, 60.0),
        vec2(-40.0, 25.0),
        vec2(40.0, 25.0),
    ];

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
    let muscles = vec![[5, 0], [6, 0]];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}
