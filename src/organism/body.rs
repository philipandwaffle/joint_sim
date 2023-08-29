use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{brain::Brain, joint::Joint, muscle::Muscle};

#[derive(Resource)]
pub struct OrganismList {
    pub organisms: Vec<Organism>,
}

#[derive(Resource)]
pub struct Organism {
    pub brain: Brain,
    pub muscles: Vec<Muscle>,
}
impl Organism {
    pub fn new(brain_structure: Vec<usize>, muscles: Vec<Muscle>) -> Self {
        let in_neurones = brain_structure[0];
        let out_neurones = brain_structure[brain_structure.len() - 1];
        let num_muscles = muscles.len();

        if in_neurones > num_muscles {
            panic!("There are not enough neurons in the input layer");
        } else if out_neurones != num_muscles {
            panic!("There are not enough neurons in the output layer");
        }

        return Self {
            brain: Brain::new(brain_structure, |x| f32::tanh(x)),
            muscles: muscles,
        };
    }

    pub fn tick_brain(&mut self) {
        let prev_muscle_state = self
            .muscles
            .iter()
            .map(|m| m.impulse_scale)
            .collect::<Vec<f32>>();
        let cur_muscle_state = self.brain.feed_forward(prev_muscle_state);

        for i in 0..cur_muscle_state.len() {
            self.muscles[i].impulse_scale = cur_muscle_state[0];
        }
    }
}

pub fn update_muscles(
    bodies: Res<OrganismList>,
    mut muscles: Query<(&mut ExternalImpulse, &Transform), With<Joint>>,
) {
    for body in bodies.organisms.iter() {
        println!(
            "{:?}",
            body.muscles
                .iter()
                .map(|x| x.impulse_scale)
                .collect::<Vec<f32>>()
        );
        for muscle in body.muscles.iter() {
            let [(mut a_ei, a_t), (mut b_ei, b_t)] = muscles.get_many_mut(muscle.joints).unwrap();
            let impulse_scale = muscle.impulse_scale * 2500.0;
            let dir = b_t.translation.truncate() - a_t.translation.truncate();
            let impulse = dir.normalize() * impulse_scale;

            a_ei.impulse = impulse;
            b_ei.impulse = -impulse;
        }
    }
}

pub fn update_brains(mut bodies: ResMut<OrganismList>) {
    for body in bodies.organisms.iter_mut() {
        body.tick_brain();
    }
}
