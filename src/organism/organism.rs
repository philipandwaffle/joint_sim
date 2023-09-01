use bevy::{
    prelude::{Commands, Entity, Resource, Vec2},
    transform::commands,
};

use super::{bone::Bone, brain::Brain, genome::Genome, joint::JointBundle, muscle::Muscle};

#[derive(Clone)]
pub struct OrganismBuilder {
    brain: Brain,
    genome: Genome,
    joint_pos: Vec<Vec2>,
    bones: Vec<[usize; 2]>,
    muscles: Vec<[usize; 2]>,
}
impl OrganismBuilder {
    pub fn new(
        external_stimuli_count: usize,
        brain_hidden_structure: Vec<usize>,
        joint_pos: Vec<Vec2>,
        bones: Vec<[usize; 2]>,
        muscles: Vec<[usize; 2]>,
    ) -> Self {
        let num_muscles = muscles.len();
        let mut brain_structure = vec![(num_muscles * 2) + 1 + external_stimuli_count];
        brain_structure.extend(brain_hidden_structure.iter());
        brain_structure.push(num_muscles);

        return Self {
            brain: Brain::new(brain_structure, |x| f32::tanh(x)),
            genome: Genome::default(),
            joint_pos,
            bones,
            muscles,
        };
    }

    pub fn spawn(&self, commands: &mut Commands, translation: Vec2) -> Organism {
        let num_muscles = self.muscles.len();
        let mut joint_ents = Vec::with_capacity(self.joint_pos.len());
        // let mut bone_ents = Vec::with_capacity(bones.len());
        let mut muscles_ents = Vec::with_capacity(num_muscles);

        for jp in self.joint_pos.iter() {
            let ent = commands
                .spawn(JointBundle::from_translation(translation + *jp))
                .id();
            joint_ents.push(ent);
        }

        for [j_a, j_b] in self.bones.iter() {
            let b = Bone::new(
                commands,
                [joint_ents[*j_a], joint_ents[*j_b]],
                [self.joint_pos[*j_a], self.joint_pos[*j_b]],
                None,
            );
            // bone_ents.push(b);
        }

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

    pub fn mutate(&mut self) {
        self.genome.mutate();
        self.brain.mutate(
            self.genome.learning_rate.val,
            self.genome.learning_factor.val,
        );
    }
}

#[derive(Resource, Clone)]
pub struct Organism {
    pub brain: Brain,
    pub genome: Genome,
    pub joints: Vec<Entity>,
    pub muscles: Vec<Muscle>,
    pub freeze_progress: f32,
}

impl Organism {
    pub fn despawn(&self, commands: &mut Commands) {
        for j in self.joints.iter() {
            commands.get_entity(*j).unwrap().despawn();
        }
    }

    pub fn process_stimuli(&mut self, mut stimuli: Vec<f32>) {
        // 0th = time
        let a = f32::sqrt(self.genome.internal_clock.val);
        let x = stimuli[0] / a;
        stimuli[0] = (2.0 * x.rem_euclid(a) / a) - 1.0;

        self.tick_brain(stimuli);
    }

    fn tick_brain(&mut self, stimuli: Vec<f32>) {
        let brain_out = self.brain.feed_forward(stimuli);
        self.brain.set_memory(brain_out.clone());
        for i in 0..brain_out.len() {
            self.muscles[i].len_modifier = brain_out[0];
        }
    }
}
