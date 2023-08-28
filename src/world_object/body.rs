use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{joint::Joint, muscle::Muscle};

#[derive(Resource)]
pub struct BodyList {
    pub bodies: Vec<Body>,
}

#[derive(Resource)]
pub struct Body {
    pub muscles: Vec<Muscle>,
}

pub fn handle_bodies(
    bodies: Res<BodyList>,
    mut muscles: Query<(&mut ExternalImpulse, &Transform), With<Joint>>,
) {
    for body in bodies.bodies.iter() {
        for muscle in body.muscles.iter() {
            let [(mut a_ei, a_t), (mut b_ei, b_t)] = muscles.get_many_mut(muscle.joints).unwrap();
            let impulse_scale = muscle.impulse_scale;
            let dir = b_t.translation.truncate() - a_t.translation.truncate();
            let impulse = dir.normalize() * impulse_scale;

            a_ei.impulse = impulse;
            b_ei.impulse = -impulse;
        }
    }
}
