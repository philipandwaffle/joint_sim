use bevy::{
    math::vec2,
    prelude::{Commands, Handle, Quat, Query, Res, ResMut, Resource, Transform, With, Without},
    sprite::ColorMaterial,
    time::Time,
};
use bevy_rapier2d::prelude::{Damping, ExternalImpulse};

use crate::{config::structs::GenerationConfig, handles::Handles};

use super::{
    bone::Bone,
    helper_fn::{quat_to_vec2, vec2_z_rot},
    joint::Joint,
    muscle::Muscle,
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

    pub fn set_builders(&mut self, builders: Vec<OrganismBuilder>) {
        self.builders = builders;
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
    pub fn spawn(&mut self, commands: &mut Commands, handles: &Handles, vertical_sep: f32) {
        let mut cur_translation = vec2(0.0, vertical_sep * 0.15);

        // Pre-allocate organisms vec
        // self.organisms.clear();
        let num_organisms = self.builders.len();
        self.organisms = Vec::with_capacity(num_organisms);

        // Spawn and push organism to vec
        for i in 0..num_organisms {
            self.organisms
                .push(self.builders[i].spawn(commands, &handles, cur_translation));
            // .push(self.builders[i].spawn(commands, vec2(0.0, 0.0), i as u32));
            cur_translation.y += vertical_sep;
        }
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
        let damping = match x >= 1.0 {
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
                    d.linear_damping = damping;
                    // d.linear_damping = damping;
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
    handles: Res<Handles>,
    mut bones: Query<(&mut ExternalImpulse, &Transform), With<Bone>>,
    mut muscles: Query<(&Muscle, &mut Transform, &mut Handle<ColorMaterial>), Without<Bone>>,
) {
    // Short circuit if organisms haven't spawned;
    if !ol.is_spawned {
        return;
    }

    // let now = Instant::now();
    for (m, mut t, mut cm) in muscles.iter_mut() {
        match bones.get_many_mut(m.bones) {
            Ok([(mut a_ei, a_t), (mut b_ei, b_t)]) => {
                // readout(a_t, b_t);
                // Apply impulse to joints
                let a_pos = a_t.translation.truncate();
                let b_pos = b_t.translation.truncate();
                let ab = b_pos - a_pos;
                let len = ab.length();
                let target_len = m.get_target_len();
                let diff = target_len - len;

                let contract_expand = if diff > 0.01 {
                    *cm = handles.muscle_contract_material.clone();
                    1.0
                } else if diff < -0.01 {
                    *cm = handles.muscle_expand_material.clone();
                    -1.0
                } else {
                    *cm = handles.muscle_neutral_material.clone();
                    0.0
                };

                let modifier = 40.0;

                if diff != 0.0 {
                    a_ei.impulse = ab.normalize() * contract_expand * modifier;
                    b_ei.impulse = -ab.normalize() * contract_expand * modifier;
                }

                t.translation = (a_pos + (ab * 0.5)).extend(-0.3);
                t.rotation = Quat::from_rotation_z(vec2_z_rot(&b_pos, &a_pos));
                t.scale.y = len;
            }
            Err(_) => {
                //TODO this is dumb make system only run when bones are spawned
            }
        }
    }
}

pub fn update_brains(
    mut ol: ResMut<OrganismList>,
    gc: Res<GenerationConfig>,
    mut muscles: Query<&mut Muscle>,
    bones: Query<&Transform, With<Bone>>,
) {
    // let now = Instant::now();

    // Short circuit if organisms haven't spawned;
    if !ol.is_spawned {
        return;
    }

    // Gather global
    let elapsed_seconds = gc.timer.elapsed_secs();

    for o in ol.organisms.iter_mut() {
        let mut stimuli = Vec::with_capacity(o.brain.get_num_inputs());
        stimuli.push(elapsed_seconds);

        let mut muscled_bone_rots = Vec::with_capacity(o.muscles.len() * 4);
        for m_ent in o.muscles.iter() {
            let m = match muscles.get(*m_ent) {
                Ok(m) => m,
                Err(_) => return,
            };
            let bone_trans = match bones.get_many(m.bones) {
                Ok(b_t) => b_t,
                Err(_) => return,
            };

            let vec_a = quat_to_vec2(&bone_trans[0].rotation);
            let vec_b = quat_to_vec2(&bone_trans[1].rotation);

            muscled_bone_rots.push(vec_a.x);
            muscled_bone_rots.push(vec_a.y);
            muscled_bone_rots.push(vec_b.x);
            muscled_bone_rots.push(vec_b.y);
        }

        stimuli.extend(muscled_bone_rots);

        // Process stimuli
        let brain_out = o.process_stimuli(&mut stimuli);

        for i in 0..brain_out.len() {
            let cur_len_modifier = &mut muscles.get_mut(o.muscles[i]).unwrap().len_modifier;
            o.energy_used += (*cur_len_modifier - brain_out[i]).abs().sqrt();
            *cur_len_modifier = brain_out[i];
        }
        o.brain.set_memory(brain_out);
    }

    // println!("processing stimuli took {:?}", total_brain_process);
    // println!("update_brains took {:?}", now.elapsed());
}
