use bevy::{
    prelude::{
        resource_exists, App, Commands, IntoSystemConfigs, Plugin, PreStartup, Query, Res, ResMut,
        Startup, Transform, Update, With,
    },
    time::Time,
};

use nalgebra::ComplexField;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::{fs::File, io::BufReader};

use self::organism_builders::get_runner_v6;
use crate::{
    config::structs::{GenerationConfig, SaveConfig},
    controls::control_state::ControlState,
    handles::Handles,
    organism::{
        joint::Joint,
        organism::{Organism, OrganismBuilder},
        organism_list::OrganismList,
    },
};

pub mod environment;
mod organism_builders;

pub struct GenerationPlugin;
impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrganismList::new()).add_systems(
            Update,
            (handle_generation).run_if(resource_exists::<OrganismList>()),
        );
    }
}

pub fn handle_generation(
    mut commands: Commands,
    mut gc: ResMut<GenerationConfig>,
    sc: Res<SaveConfig>,
    time: Res<Time>,
    handles: Res<Handles>,
    mut ol: ResMut<OrganismList>,
    mut cs: ResMut<ControlState>,
    joint_transforms: Query<&Transform, With<Joint>>,
) {
    gc.timer.tick(time.delta());
    let elapsed_secs = gc.timer.elapsed_secs();

    if ol.builders.is_empty() {
        return;
    }

    if gc.unfreeze_flag && elapsed_secs > 0.1 {
        gc.unfreeze_flag = false;
        ol.unfreeze();
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

        // Despawn current generation
        ol.despawn(&mut commands);

        // Spawn new generation
        ol.set_builders(new_builders);
        ol.spawn(&mut commands, &handles, gc.vertical_sep);
    }
}

fn get_next_generation_builders(
    ol: &mut OrganismList,
    gc: &mut GenerationConfig,
    joint_transforms: &Query<&Transform, With<Joint>>,
) -> Vec<OrganismBuilder> {
    // Calculate fitness
    let num_organisms = gc.num_organisms;

    // Pick the 'best' organisms
    let fitness = calc_fitness(&ol.organisms, num_organisms, joint_transforms);

    let mut rng = rand::thread_rng();
    let mut new_builders = Vec::with_capacity(num_organisms);

    while new_builders.len() <= num_organisms / 2 {
        for i in 0..num_organisms {
            let fit = fitness[i];

            if fit.abs() >= rng.gen::<f32>() {
                new_builders.push(ol.builders[i].clone());
            }
        }
    }

    let sample = Uniform::from(0..new_builders.len());
    while new_builders.len() < num_organisms {
        let index = sample.sample(&mut rng);
        let new_builder = new_builders[index].clone();
        new_builders.push(new_builder);
    }

    // Mutate each organism
    new_builders.iter_mut().for_each(|x| x.mutate(&mut rng));

    gc.cur_generation += 1;
    return new_builders;
}

fn calc_fitness(
    organisms: &Vec<Organism>,
    num_organisms: usize,
    joint_transforms: &Query<&Transform, With<Joint>>,
) -> Vec<f32> {
    let mut fitness = Vec::with_capacity(num_organisms);

    let pos_score = organisms
        .iter()
        .map(|o| {
            o.joints
                .iter()
                .map(|x| joint_transforms.get(*x).unwrap().translation.x.max(0.0))
                .sum::<f32>()
                / o.joints.len() as f32
        })
        .collect::<Vec<f32>>();
    let max_pos_score = pos_score
        .iter()
        .max_by(|a, b| a.abs().total_cmp(&b.abs()))
        .unwrap();
    let normalised_pos_score = pos_score
        .iter()
        .map(|x| x / max_pos_score)
        .collect::<Vec<f32>>();

    let eff_score = organisms
        .iter()
        .map(|o| o.energy_used)
        .collect::<Vec<f32>>();
    let max_eff_score = eff_score
        .iter()
        .max_by(|a, b| a.abs().total_cmp(&b.abs()))
        .unwrap();
    let normalised_eff_score = pos_score
        .iter()
        .map(|x| x / max_eff_score)
        .collect::<Vec<f32>>();

    for i in 0..num_organisms {
        fitness.push(normalised_pos_score[i] * 0.5 + normalised_eff_score[i] * 0.5)
    }

    return fitness;
}

fn setup_organism_list(
    mut commands: Commands,
    handles: Res<Handles>,
    gc: Res<GenerationConfig>,
    sc: Res<SaveConfig>,
) {
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
            builders.push(get_runner_v6());
        }
    }

    let mut ol = OrganismList {
        builders: builders,
        organisms: vec![],
        is_spawned: false,
    };

    ol.spawn(&mut commands, &handles, gc.vertical_sep);
    commands.insert_resource(ol);
}
