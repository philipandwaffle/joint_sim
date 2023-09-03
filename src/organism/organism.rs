use bevy::{
    math::vec2,
    prelude::{Commands, Entity, Resource, Vec2},
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{bone::Bone, brain::Brain, genome::Genome, joint::JointBundle, muscle::Muscle};

// Acts as a blueprint for organisms so mutations can occur before spawning
#[derive(Clone, Serialize, Deserialize)]
pub struct OrganismBuilder {
    brain: Brain,
    genome: Genome,
    joint_pos: Vec<Vec2>,
    bones: Vec<[usize; 2]>,
    muscles: Vec<[usize; 2]>,
}
impl OrganismBuilder {
    // Create new builder
    pub fn new(
        external_stimuli_count: usize,
        brain_hidden_structure: Vec<usize>,
        joint_pos: Vec<Vec2>,
        bones: Vec<[usize; 2]>,
        muscles: Vec<[usize; 2]>,
    ) -> Self {
        // Get num muscles
        let num_muscles = muscles.len();

        // Calculate brain structure
        let mut brain_structure = vec![(num_muscles * 2) + 1 + external_stimuli_count];
        brain_structure.extend(brain_hidden_structure.iter());
        brain_structure.push(num_muscles);

        return Self {
            brain: Brain::new(brain_structure),
            genome: Genome::default(),
            joint_pos,
            bones,
            muscles,
        };
    }

    // Spawn the organism with an translation
    pub fn spawn(&self, commands: &mut Commands, translation: Vec2) -> Organism {
        let num_muscles = self.muscles.len();

        // Pre-allocate vectors
        let mut joint_ents = Vec::with_capacity(self.joint_pos.len());
        let mut muscles_ents = Vec::with_capacity(num_muscles);

        // Create a joint for each position supplied
        for jp in self.joint_pos.iter() {
            let ent = commands
                .spawn(JointBundle::from_translation(translation + *jp))
                .id();
            joint_ents.push(ent);
        }

        // Create a bone for each bone given
        for [j_a, j_b] in self.bones.iter() {
            Bone::new(
                commands,
                [joint_ents[*j_a], joint_ents[*j_b]],
                [self.joint_pos[*j_a], self.joint_pos[*j_b]],
            );
        }

        // Create a muscle for each muscle given
        for [j_a, j_b] in self.muscles.iter() {
            let m = Muscle::new(
                [joint_ents[*j_a], joint_ents[*j_b]],
                [self.joint_pos[*j_a], self.joint_pos[*j_b]],
            );
            muscles_ents.push(m);
        }

        return Organism {
            brain: self.brain.clone(),
            genome: self.genome.clone(),
            joints: joint_ents,
            muscles: muscles_ents,
            freeze_progress: 0.0,
        };
    }

    // Mutate the builder
    pub fn mutate(&mut self) {
        // Mutate genome
        self.genome.mutate();

        // Mutate brain
        self.brain.mutate(
            self.genome.learning_rate.val,
            self.genome.learning_factor.val,
        );

        // Mutate joint positions
        let mut rng = rand::thread_rng();
        for j_pos in self.joint_pos.iter_mut() {
            if rng.gen::<f32>() <= self.genome.joint_mutate_rate.val {
                let mf = self.genome.joint_mutate_factor.val;
                let dx = rng.gen_range(-mf..mf);
                let dy = rng.gen_range(-mf..mf);

                let unclamped = j_pos.clone() + vec2(dx, dy);
                *j_pos = unclamped.clamp(vec2(-100.0, 0.0), vec2(100.0, 200.0));
            }
        }
    }
}

// Container for the components making up an organism
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Organism {
    pub brain: Brain,
    pub genome: Genome,
    pub joints: Vec<Entity>,
    pub muscles: Vec<Muscle>,
    pub freeze_progress: f32,
}

impl Organism {
    // Despawn all entities associated with the organism
    pub fn despawn(&self, commands: &mut Commands) {
        for j in self.joints.iter() {
            commands.get_entity(*j).unwrap().despawn();
        }
    }

    // Take input stimuli and tick the brain
    pub fn process_stimuli(&mut self, mut stimuli: Vec<f32>) {
        // normalise the 0th input, time
        let a = self.genome.internal_clock.val;
        let x = stimuli[0] / a;
        stimuli[0] = (2.0 * x.rem_euclid(a) / a) - 1.0;

        // Make brain process stimuli
        self.tick_brain(stimuli);
    }

    // Process stimuli and alter muscles
    fn tick_brain(&mut self, stimuli: Vec<f32>) {
        // Calculate brain out
        let brain_out = self.brain.feed_forward(stimuli);

        // Alter muscle length
        for i in 0..brain_out.len() {
            self.muscles[i].len_modifier = brain_out[i];
        }
    }
}
