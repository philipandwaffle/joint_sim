use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    bone::Bone,
    brain::Brain,
    joint::{Joint, JointBundle},
    muscle::Muscle,
};

#[derive(Resource)]
pub struct OrganismList {
    pub organisms: Vec<Organism>,
}
impl OrganismList {
    pub fn new() -> Self {
        return Self { organisms: vec![] };
    }
    pub fn push(&mut self, o: Organism) {
        self.organisms.push(o);
    }

    pub fn toggle_freeze(&mut self) {
        for o in self.organisms.iter_mut() {
            o.queue_freeze = true;
        }
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        for o in self.organisms.iter() {
            o.despawn(commands);
        }
        self.organisms = vec![];
    }
}

#[derive(Resource, Clone)]
pub struct Organism {
    pub brain: Brain,
    pub joints: Vec<Entity>,
    pub muscles: Vec<Muscle>,
    pub frozen: bool,
    queue_freeze: bool,
}

impl Organism {
    pub fn new(
        commands: &mut Commands,
        offset: Vec2,
        brain_structure: Vec<usize>,
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

        let mut structure = vec![num_muscles];
        structure.extend(brain_structure.iter());
        structure.push(num_muscles);

        return Self {
            brain: Brain::new(structure, 1, |x| f32::tanh(x)),
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

    pub fn tick_brain(&mut self, mut memory: Vec<f32>) {
        memory.append(
            &mut self
                .muscles
                .iter()
                .map(|m| m.len_modifier)
                .collect::<Vec<f32>>(),
        );
        let cur_muscle_state = self.brain.feed_forward(memory);

        for i in 0..cur_muscle_state.len() {
            self.muscles[i].len_modifier = cur_muscle_state[0];
        }
    }
}

pub fn freeze_queued(mut ol: ResMut<OrganismList>, mut joints: Query<&mut Damping, With<Joint>>) {
    for o in ol.organisms.iter_mut() {
        if !o.queue_freeze {
            return;
        }
        o.queue_freeze = false;

        o.frozen = !o.frozen;
        let linear_damping = match o.frozen {
            true => 1000.0,
            false => 0.5,
        };

        for j in o.joints.iter_mut() {
            joints.get_mut(*j).unwrap().linear_damping = linear_damping;
        }
    }
}

pub fn update_muscles(
    ol: Res<OrganismList>,
    mut muscles: Query<(&mut ExternalImpulse, &Transform), With<Joint>>,
) {
    let cur_id = 0;
    for i in 0..ol.organisms.len() {
        let body = &ol.organisms[i];
        if i == cur_id {
            println!(
                "{:?}",
                body.muscles
                    .iter()
                    .map(|x| x.len_modifier)
                    .collect::<Vec<f32>>()
            );
        }

        for muscle in body.muscles.iter() {
            let [(mut a_ei, a_t), (mut b_ei, b_t)] = muscles.get_many_mut(muscle.joints).unwrap();
            let dir = b_t.translation.truncate() - a_t.translation.truncate();
            let diff = dir.length() - muscle.get_target_len();
            let modifier = 20.0;
            if diff != 0.0 {
                a_ei.impulse = dir * -diff * modifier;
                b_ei.impulse = dir * diff * modifier;
            }
            // let impulse_scale = muscle.impulse_scale * 2500.0;
            // let dir = b_t.translation.truncate() - a_t.translation.truncate();
            // let impulse = dir.normalize() * impulse_scale;

            // a_ei.impulse = impulse;
            // b_ei.impulse = -impulse;
        }
    }
}

pub fn update_brains(mut ol: ResMut<OrganismList>, time: Res<Time>) {
    let a = f32::sqrt(5.0);
    let x = time.elapsed_seconds() / a;
    let time_mem0 = (2.0 * x.rem_euclid(a) / a) - 1.0;
    let memory = vec![time_mem0];
    // println!("mem block {:?}", memory);
    for body in ol.organisms.iter_mut() {
        body.tick_brain(memory.clone());
    }
}
