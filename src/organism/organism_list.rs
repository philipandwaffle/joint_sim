use std::{f32::consts::PI, sync::Arc, thread};

use bevy::{
    math::vec2,
    prelude::{Color, Commands, Quat, Query, Res, ResMut, Resource, Transform, With, Without},
    time::Time,
    utils::Instant,
};
use bevy_inspector_egui::egui::mutex::Mutex;
use bevy_prototype_lyon::prelude::Fill;
use bevy_rapier2d::prelude::{Damping, ExternalImpulse};

use crate::{config::structs::GenerationConfig, organism::bone};

use super::{
    bone::Bone,
    helper_fn::{quat_z_rot, vec2_z_rot},
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
        let mut cur_translation = vec2(0.0, vertical_sep * 0.5);

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
    mut bones: Query<(&mut ExternalImpulse, &Transform), With<Bone>>,
    mut muscles: Query<(&Muscle, &mut Transform, &mut Fill), Without<Bone>>,
) {
    let now = Instant::now();
    for (m, mut t, mut f) in muscles.iter_mut() {
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

                let mut foo = 0.0;
                if diff > 0.0 {
                    foo = 1.0;
                    f.color = Color::RED;
                } else if diff < 0.0 {
                    foo = -1.0;
                    f.color = Color::BLUE;
                }
                let modifier = 10.0;
                // let modifier = 0.0;

                if diff != 0.0 {
                    a_ei.impulse = ab * foo * modifier;
                    b_ei.impulse = -ab * foo * modifier;
                }

                let r = quat_z_rot(a_t.rotation);
                t.rotation = Quat::from_rotation_z(vec2_z_rot(b_pos, a_pos) - r);
                let y_scale = len / m.base_len;
                // let y_scale = 1.0 * (1.0 + m.len_modifier);
                t.scale.y = y_scale;
            }
            Err(_) => {
                //TODO this is dumb make system only run when bones are spawned
                return;
            }
        }
    }
    println!("update_muscles took {:?}", now.elapsed());
}

fn readout(a: &Transform, b: &Transform) {
    let a_rot = quat_z_rot(a.rotation) * 180.0 / PI;
    let b_rot = quat_z_rot(b.rotation) * 180.0 / PI;

    let norm_a_rot = f32::acos(a.rotation.dot(Quat::IDENTITY)) * 180.0 / PI;
    let norm_b_rot = f32::acos(b.rotation.dot(Quat::IDENTITY)) * 180.0 / PI;

    // let id_a_rot = quat_z_rot(Quat::IDENTITY - a.rotation) * 180.0 / PI;
    // let id_b_rot = quat_z_rot(Quat::IDENTITY - b.rotation) * 180.0 / PI;

    println!(
        "a_rot: {}, b_rot:{}, a_norm: {}, b_norm: {}",
        a_rot, b_rot, norm_a_rot, norm_b_rot
    );
}

// Make brains process stimuli
pub fn update_brains2(
    mut ol: ResMut<OrganismList>,
    gc: Res<GenerationConfig>,
    mut muscles: Query<&mut Muscle>,
    bones: Query<&Transform, With<Bone>>,
) {
    // bones.iter()
    let now = Instant::now();

    // Short circuit if organisms haven't spawned;
    if !ol.is_spawned {
        return;
    }

    // Gather global
    let elapsed_seconds = gc.timer.elapsed_secs();
    let mut external_stimuli = Vec::with_capacity(ol.organisms[0].brain.get_num_inputs());
    external_stimuli.push(elapsed_seconds);

    let num_organisms = gc.num_organisms;
    let num_threads = 12;
    let batch_size = num_organisms / num_threads;
    let mut rem = num_organisms % num_threads;

    let mut org_bone_data = Vec::with_capacity(num_organisms);
    for o in ol.organisms.iter() {
        let mut bone_data = Vec::with_capacity(o.bones.len());
        for b in o.bones.iter() {
            match bones.get(*b) {
                Ok(t) => bone_data.push(quat_z_rot(t.rotation)),
                Err(_) => return,
            }
        }
        org_bone_data.push(bone_data);
    }

    let mut handles = Vec::with_capacity(num_threads);

    let mut cur_start = 0;
    for i in 0..num_threads {
        let mut org_bone_data = org_bone_data.clone();
        let orgs = ol.organisms.clone();
        let es_clone = external_stimuli.clone();

        let mut cur_batch_size = batch_size;
        if rem != 0 {
            cur_batch_size += 1;
            rem -= 1;
        }
        let start = cur_start;
        cur_start += cur_batch_size;
        let end = cur_start;

        let handle = thread::spawn(move || {
            println!("starting thread: {:?}", i);
            let mut brain_outs = Vec::with_capacity(end - start);

            for i in start..end {
                let mut stimuli = es_clone.clone();
                stimuli.append(&mut org_bone_data[i]);

                let o = &orgs[i];
                brain_outs.push(o.process_stimuli(&mut stimuli));
            }
            println!("finished thread: {:?}", i);
            (start, end, brain_outs)
        });
        handles.push(handle);
    }

    for h in handles {
        match h.join() {
            Ok((start, end, brain_outs)) => {
                println!("joining: {:?}, {:?}", start, end);
                for i in start..end {
                    let o = &mut ol.organisms[i];

                    for j in 0..o.muscles.len() {
                        muscles.get_mut(o.muscles[j]).unwrap().len_modifier =
                            brain_outs[i - start][j];
                    }
                    o.brain.memory = brain_outs[i - start].clone();
                }
            }
            Err(err) => todo!(),
        }
    }

    println!("update_brains took {:?}", now.elapsed());
}
pub fn update_brains(
    mut ol: ResMut<OrganismList>,
    gc: Res<GenerationConfig>,
    mut muscles: Query<&mut Muscle>,
    bones: Query<&Transform, With<Bone>>,
) {
    let now = Instant::now();

    // Short circuit if organisms haven't spawned;
    if !ol.is_spawned {
        return;
    }

    // Gather global
    let elapsed_seconds = gc.timer.elapsed_secs();
    let mut external_stimuli = Vec::with_capacity(ol.organisms[0].brain.get_num_inputs());
    external_stimuli.push(elapsed_seconds);

    // Process stimuli for each organism
    for o in ol.organisms.iter_mut() {
        let mut stimuli = external_stimuli.clone();

        for b in o.bones.iter() {
            match bones.get(b.clone()) {
                Ok(t) => {
                    stimuli.push(quat_z_rot(t.rotation));
                }
                Err(_) => {
                    //TODO this is dumb make system only run when joints are spawned
                    return;
                }
            }
        }
        // Process stimuli
        let brain_out = o.process_stimuli(&mut stimuli);
        for i in 0..brain_out.len() {
            muscles.get_mut(o.muscles[i]).unwrap().len_modifier = brain_out[i].clone();
        }
    }

    println!("update_brains took {:?}", now.elapsed());
}
