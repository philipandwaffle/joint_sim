use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{bone::BoneMotor, muscle::Muscle};

#[derive(Resource)]
pub struct BodyList {
    pub bodies: Vec<Body>,
}

#[derive(Resource)]
pub struct Body {
    pub muscles: Vec<Muscle>,
}

pub fn update_bodies(
    time: Res<Time>,
    mut bodies: ResMut<BodyList>,
    mut bones: Query<&mut BoneMotor>,
) {
    for body in bodies.bodies.iter_mut() {
        for muscle in body.muscles.iter_mut() {
            muscle.contract(&mut bones, f32::sin(time.elapsed_seconds() * 0.1) * 200.0);
        }
    }
}
