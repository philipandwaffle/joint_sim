use bevy::{
    math::vec2,
    prelude::{Commands, DespawnRecursiveExt, Entity, Resource, Vec2},
};
use rand::{rngs::ThreadRng, Rng};
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
        let mut bone_ents = Vec::with_capacity(self.bones.len());
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
            let ent = Bone::new(
                commands,
                [joint_ents[*j_a], joint_ents[*j_b]],
                [
                    translation + self.joint_pos[*j_a],
                    translation + self.joint_pos[*j_b],
                ],
            );
            bone_ents.push(ent);
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
            bones: bone_ents,
            muscles: muscles_ents,
            freeze_progress: 0.0,
        };
    }

    // Mutate the builder
    pub fn mutate(&mut self, rng: &mut ThreadRng) {
        // Mutate genome
        self.genome.mutate(rng);

        // Mutate brain
        self.brain
            .mutate(rng, self.genome.lr.val, self.genome.lf.val);

        // Mutate joint positions
        for i in 0..self.joint_pos.len() {
            if rng.gen::<f32>() <= self.genome.joint_mr.val {
                let mf = self.genome.joint_mf.val;
                self.move_joint(rng, i, mf);
            }
        }

        // Add/remove bone
        if rng.gen::<f32>() <= self.genome.bone_mr.val {
            match rng.gen::<f32>() <= 0.5 {
                true => self.add_bone(rng, self.genome.muscle_mr.val),
                false => self.remove_bone(rng),
            }
        }

        // Add/remove muscle
        if rng.gen::<f32>() <= self.genome.muscle_mr.val {
            match rng.gen::<f32>() <= 0.5 {
                true => self.add_muscle(rng),
                false => self.remove_muscle(rng),
            }
        }
    }

    pub fn move_joint(&mut self, rng: &mut ThreadRng, i: usize, mf: f32) {
        let dx = rng.gen_range(-mf..mf);
        let dy = rng.gen_range(-mf..mf);
        let unclamped = self.joint_pos[i] + vec2(dx, dy);
        self.joint_pos[i] = unclamped.clamp(vec2(-100.0, 0.0), vec2(100.0, 200.0));
    }

    pub fn add_bone(&mut self, rng: &mut ThreadRng, mf: f32) {
        let index = rng.gen::<usize>();

        let joint_pos =
            vec2(rng.gen_range(-mf..mf), rng.gen_range(-mf..mf)) + self.joint_pos[index];
        self.joint_pos.push(joint_pos);

        let bone = [self.joint_pos.len(), index];
        self.bones.push(bone);
    }

    pub fn remove_bone(&mut self, rng: &mut ThreadRng) {
        let num_bones = self.bones.len();
        if num_bones == 0 {
            return;
        }

        let index = rng.gen_range(0..num_bones);
        self.bones.remove(index);
    }

    pub fn add_muscle(&mut self, rng: &mut ThreadRng) {
        let num_joints = self.joint_pos.len();
        if num_joints < 2 {
            return;
        }

        let a = rng.gen_range(0..num_joints);
        let mut b = rng.gen_range(0..num_joints);

        while a == b {
            b = rng.gen_range(0..self.joint_pos.len());
        }

        self.brain.add_io();
        self.muscles.push([a, b]);
    }

    pub fn remove_muscle(&mut self, rng: &mut ThreadRng) {
        let num_muscles = self.muscles.len();
        if num_muscles == 0 {
            return;
        }
        let index = rng.gen_range(0..num_muscles);

        self.brain.remove_io();
        self.muscles.remove(index);
    }
}

// Container for the components making up an organism
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Organism {
    pub brain: Brain,
    pub genome: Genome,
    pub joints: Vec<Entity>,
    pub muscles: Vec<Muscle>,
    pub bones: Vec<Entity>,
    pub freeze_progress: f32,
}

impl Organism {
    // Despawn all entities associated with the organism
    pub fn despawn(&self, commands: &mut Commands) {
        for j in self.joints.iter() {
            commands.get_entity(*j).unwrap().despawn_recursive();
        }
        for b in self.bones.iter() {
            commands.get_entity(*b).unwrap().despawn_recursive();
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
