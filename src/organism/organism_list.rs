use bevy::{
    math::vec2,
    prelude::{Commands, EulerRot, Query, Res, ResMut, Resource, Transform, With},
    time::Time,
    transform::commands,
};
use bevy_rapier2d::prelude::{Damping, ExternalImpulse};

use crate::generation::config::GenerationConfig;

use super::{
    joint::Joint,
    organism::{Organism, OrganismBuilder},
};

#[derive(Resource)]
pub struct OrganismList {
    pub builders: Vec<OrganismBuilder>,
    pub organisms: Vec<Organism>,
}
impl OrganismList {
    pub fn new() -> Self {
        return Self {
            builders: vec![],
            organisms: vec![],
        };
    }
    pub fn push(&mut self, o: Organism) {
        self.organisms.push(o);
    }

    pub fn toggle_freeze(&mut self) {
        for o in self.organisms.iter_mut() {
            o.freeze_progress = 0.0;
        }
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        for o in self.organisms.iter() {
            o.despawn(commands);
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, vertical_sep: f32) {
        let mut cur_translation = vec2(0.0, 0.0);
        self.organisms = Vec::with_capacity(self.builders.len());
        for i in 0..self.builders.len() {
            self.organisms
                .push(self.builders[i].spawn(commands, cur_translation));
            cur_translation.y += vertical_sep;
        }
    }
}

pub fn freeze_queued(
    mut ol: ResMut<OrganismList>,
    mut joints: Query<&mut Damping, With<Joint>>,
    time: Res<Time>,
) {
    for o in ol.organisms.iter_mut() {
        if o.freeze_progress == -1.0 {
            continue;
        }
        o.freeze_progress += time.delta_seconds();

        let x = o.freeze_progress;
        let linear_damping = match x >= 1.0 {
            true => {
                o.freeze_progress = -1.0;
                0.2
            }
            false => 1000.0 * f32::powf(x - 1.0, 2.0) + 0.2,
        };

        for j in o.joints.iter_mut() {
            match joints.get_mut(*j) {
                Ok(mut d) => {
                    d.linear_damping = linear_damping;
                }
                Err(_) => {
                    //TODO this is dumb make system only run when joints are spawned
                    o.freeze_progress = 1.0;
                    continue;
                }
            }
        }
    }
}

pub fn update_muscles(
    ol: Res<OrganismList>,
    mut muscles: Query<(&mut ExternalImpulse, &Transform), With<Joint>>,
) {
    let cur_id = -1;
    for i in 0..ol.organisms.len() {
        let body = &ol.organisms[i];
        if i as i32 == cur_id {
            println!(
                "{:?}",
                body.muscles
                    .iter()
                    .map(|x| x.len_modifier)
                    .collect::<Vec<f32>>()
            );
        }

        for muscle in body.muscles.iter() {
            match muscles.get_many_mut(muscle.joints) {
                Ok([(mut a_ei, a_t), (mut b_ei, b_t)]) => {
                    let dir = b_t.translation.truncate() - a_t.translation.truncate();
                    let diff = dir.length() - muscle.get_target_len();
                    let modifier = 2.0;
                    if diff != 0.0 {
                        a_ei.impulse = dir * diff * modifier;
                        b_ei.impulse = dir * -diff * modifier;
                    }
                }
                Err(_) => {
                    //TODO this is dumb make system only run when joints are spawned
                    return;
                }
            }
        }
    }
}

pub fn update_brains(
    mut ol: ResMut<OrganismList>,
    config: Res<GenerationConfig>,
    joints: Query<&Transform, With<Joint>>,
) {
    let elasped_seconds = config.timer.elapsed_secs();
    let external_stimuli = vec![elasped_seconds];

    for body in ol.organisms.iter_mut() {
        let mut stimuli = external_stimuli.clone();

        match joints.get(body.joints[0]) {
            Ok(j) => {
                let j_rotation = j.rotation.to_euler(EulerRot::ZYX).0;
                stimuli.push(j_rotation);
            }
            Err(_) => {
                //TODO this is dumb make system only run when joints are spawned
                return;
            }
        }

        for m in body.muscles.iter() {
            match joints.get(m.joints[0]) {
                Ok(j) => {
                    let j_rotation = j.rotation.to_euler(EulerRot::ZYX).0;
                    stimuli.push(j_rotation);
                }
                Err(_) => {
                    //TODO this is dumb make system only run when joints are spawned
                    return;
                }
            }
        }
        body.process_stimuli(stimuli.clone());
    }
}
