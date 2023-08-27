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

fn test_body(
    time: Res<Time>,
    bodies: Res<BodyList>,
    mut muscles: Query<(&mut Velocity, &RigidBody, &Transform), With<Joint>>,
) {
    for body in bodies.bodies.iter() {
        for muscle_ents in body.muscles.iter() {
            let [(mut a_v, a_rb, a_t), (mut b_v, b_rb, b_t)] =
                muscles.get_many_mut(*muscle_ents).unwrap();
            let speed = f32::sin(time.elapsed_seconds());

            let dir = b_t.translation - a_t.translation;
        }
    }
}
