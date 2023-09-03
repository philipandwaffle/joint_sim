use bevy::{
    math::vec2,
    prelude::{Commands, Quat, Query, Res, ResMut, Resource, Transform, With},
    time::Time,
};
use bevy_rapier2d::prelude::{Damping, ExternalImpulse};

use crate::generation::config::GenerationConfig;

use super::{
    joint::Joint,
    organism::{Organism, OrganismBuilder},
};

// Contains every organism
#[derive(Resource)]
pub struct OrganismList {
    pub builders: Vec<OrganismBuilder>,
    pub organisms: Vec<Organism>,
    pub is_spawned: bool,
}
impl OrganismList {
    pub fn new() -> Self {
        return Self {
            builders: vec![],
            organisms: vec![],
            is_spawned: false,
        };
    }
    pub fn push(&mut self, o: Organism) {
        self.organisms.push(o);
    }

    // Sets the freeze progress to 0
    pub fn unfreeze(&mut self) {
        // Set the freeze progress to 0 for every organism
        for o in self.organisms.iter_mut() {
            o.freeze_progress = 0.0;
        }
    }

    // Despawn every organism
    pub fn despawn(&mut self, commands: &mut Commands) {
        self.is_spawned = false;
        for o in self.organisms.iter() {
            o.despawn(commands);
        }
    }

    // Spawn every organism using the builders
    pub fn spawn(&mut self, commands: &mut Commands, vertical_sep: f32) {
        let mut cur_translation = vec2(0.0, 0.0);

        // Pre-allocate organisms vec
        self.organisms = Vec::with_capacity(self.builders.len());

        // Spawn and push organism to vec
        for i in 0..self.builders.len() {
            self.organisms
                .push(self.builders[i].spawn(commands, cur_translation));
            cur_translation.y += vertical_sep;
        }

        // Set list as spawned
        self.is_spawned = true;
    }
}

// System to unfreeze organisms
pub fn unfreeze_queued(
    mut ol: ResMut<OrganismList>,
    mut joints: Query<&mut Damping, With<Joint>>,
    time: Res<Time>,
) {
    //TODO move freeze progress to OrganismList so each Organism doesn't need individually checked

    // Loop through each organism
    for o in ol.organisms.iter_mut() {
        // Skip if unfrozen
        if o.freeze_progress == -1.0 {
            continue;
        }
        o.freeze_progress += time.delta_seconds();

        let x = o.freeze_progress;
        // Calc linear damping
        let linear_damping = match x >= 1.0 {
            true => {
                o.freeze_progress = -1.0;
                0.2
            }
            false => 1000.0 * f32::powf(x - 1.0, 2.0) + 0.2,
        };

        // Update damping for each joint
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

// Update muscle lengths
pub fn update_muscles(
    ol: Res<OrganismList>,
    mut muscles: Query<(&mut ExternalImpulse, &Transform), With<Joint>>,
) {
    // Loop through each organisms
    for i in 0..ol.organisms.len() {
        let o = &ol.organisms[i];

        // Loop through each muscle
        for muscle in o.muscles.iter() {
            // Get joints making up muscle
            match muscles.get_many_mut(muscle.joints) {
                Ok([(mut a_ei, a_t), (mut b_ei, b_t)]) => {
                    // Apply impulse to joints
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

// Make brains process stimuli
pub fn update_brains(
    mut ol: ResMut<OrganismList>,
    config: Res<GenerationConfig>,
    joints: Query<&Transform, With<Joint>>,
) {
    // Short circuit if organisms haven't spawned;
    if !ol.is_spawned {
        return;
    }

    // Gather global
    let elapsed_seconds = config.timer.elapsed_secs();
    let mut external_stimuli = Vec::with_capacity(ol.organisms[0].brain.get_num_inputs());
    external_stimuli.push(elapsed_seconds);

    // Process stimuli for each organism
    for o in ol.organisms.iter_mut() {
        let mut stimuli = external_stimuli.clone();

        // Gather local and push to stimuli vec
        match joints.get(o.joints[0]) {
            Ok(j) => {
                // Get joint rotation
                stimuli.push(get_z_rot(j.rotation));
            }
            Err(_) => {
                //TODO this is dumb make system only run when joints are spawned
                return;
            }
        }

        // Gather local and push to stimuli vec
        for m in o.muscles.iter() {
            match joints.get(m.joints[0]) {
                Ok(j) => {
                    // Get joint rotation
                    stimuli.push(get_z_rot(j.rotation));
                }
                Err(_) => {
                    //TODO this is dumb make system only run when joints are spawned
                    return;
                }
            }
        }
        // Process stimuli
        o.process_stimuli(stimuli.clone());
    }
}

// Get z rotation from a quaternion
fn get_z_rot(q: Quat) -> f32 {
    return f32::atan2(
        2.0 * (q.w * q.z + q.x * q.y),
        1.0 - 2.0 * (q.y * q.y + q.z * q.z),
    );
}
