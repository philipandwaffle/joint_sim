use bevy::prelude::{Commands, Entity, Resource, Vec2};

use super::{bone::Bone, brain::Brain, genome::Genome, joint::JointBundle, muscle::Muscle};

#[derive(Resource, Clone)]
pub struct Organism {
    brain: Brain,
    pub genome: Genome,
    pub joints: Vec<Entity>,
    pub muscles: Vec<Muscle>,
    pub frozen: bool,
    pub queue_freeze: bool,
}

impl Organism {
    pub fn new(
        commands: &mut Commands,
        offset: Vec2,
        external_stimuli_count: usize,
        brain_hidden_structure: Vec<usize>,
        joint_pos: Vec<Vec2>,
        bones: Vec<[usize; 2]>,
        muscles: Vec<[usize; 2]>,
    ) -> Self {
        let num_muscles = muscles.len();
        let mut joint_ents = Vec::with_capacity(joint_pos.len());
        // let mut bone_ents = Vec::with_capacity(bones.len());
        let mut muscles_ents = Vec::with_capacity(num_muscles);

        for jp in joint_pos.iter() {
            let ent = commands
                .spawn(JointBundle::from_translation(offset + *jp))
                .id();
            joint_ents.push(ent);
        }

        for [j_a, j_b] in bones.iter() {
            let b = Bone::new(
                commands,
                [joint_ents[*j_a], joint_ents[*j_b]],
                [joint_pos[*j_a], joint_pos[*j_b]],
                None,
            );
            // bone_ents.push(b);
        }

        for [j_a, j_b] in muscles.iter() {
            let m = Muscle::new(
                [joint_ents[*j_a], joint_ents[*j_b]],
                [joint_pos[*j_a], joint_pos[*j_b]],
            );
            muscles_ents.push(m);
        }

        let mut brain_structure = vec![num_muscles + external_stimuli_count];
        brain_structure.extend(brain_hidden_structure.iter());
        brain_structure.push(num_muscles);

        return Self {
            brain: Brain::new(brain_structure, |x| f32::tanh(x)),
            genome: Genome::default(),
            joints: joint_ents,
            muscles: muscles_ents,
            frozen: true,
            queue_freeze: false,
        };
    }

    pub fn despawn(&self, commands: &mut Commands) {
        for j in self.joints.iter() {
            commands.get_entity(*j).unwrap().despawn();
        }
    }

    pub fn mutate(&mut self) {
        self.brain.mutate(
            self.genome.learning_rate.val,
            self.genome.learning_factor.val,
        );
        self.genome.mutate();
    }

    pub fn process_stimuli(&mut self, mut external_stimuli: Vec<f32>) {
        // 0th = time
        let a = f32::sqrt(self.genome.internal_clock.val);
        let x = external_stimuli[0] / a;
        external_stimuli[0] = (2.0 * x.rem_euclid(a) / a) - 1.0;

        self.tick_brain(external_stimuli);
    }

    fn tick_brain(&mut self, external_stimuli: Vec<f32>) {
        let brain_out = self.brain.feed_forward(external_stimuli);
        self.brain.set_memory(brain_out.clone());
        for i in 0..brain_out.len() {
            self.muscles[i].len_modifier = brain_out[0];
        }
    }
}
