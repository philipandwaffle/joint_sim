use bevy::{
    prelude::{Commands, Query, Res, ResMut, Resource, Transform, With},
    time::Time,
};
use bevy_rapier2d::prelude::{Damping, ExternalImpulse};

use super::{joint::Joint, organism::Organism};

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
            let [(mut a_ei, a_t), (mut b_ei, b_t)] = muscles.get_many_mut(muscle.joints).unwrap();
            let dir = b_t.translation.truncate() - a_t.translation.truncate();
            let diff = dir.length() - muscle.get_target_len();
            let modifier = 1.0;
            if diff != 0.0 {
                a_ei.impulse = dir * diff * modifier;
                b_ei.impulse = dir * -diff * modifier;
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
    let external_stimuli = vec![time.elapsed_seconds()];

    for body in ol.organisms.iter_mut() {
        body.process_stimuli(external_stimuli.clone());
    }
}
