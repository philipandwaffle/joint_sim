use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    bone::BoneMotor,
    muscle::{AngularMuscle, LinearMuscleMotor},
};

#[derive(Resource)]
pub struct BodyList {
    pub bodies: Vec<Body>,
}

#[derive(Resource)]
pub struct Body {
    pub angular_muscles: Vec<AngularMuscle>,
    pub linear_muscles: Vec<Entity>,
}

pub fn update_linear_muscle_velocities(
    time: Res<Time>,
    bodies: Res<BodyList>,
    mut muscles: Query<&mut LinearMuscleMotor>,
) {
    for body in bodies.bodies.iter() {
        for linear_muscle in body.linear_muscles.iter() {
            let muscle = &mut muscles.get_mut(linear_muscle.clone()).unwrap();
            muscle.apply_delta_velocity(f32::sin(time.elapsed_seconds() * 0.1) * 200.0);
        }
    }
}

pub fn update_bodies(
    time: Res<Time>,
    mut bodies: ResMut<BodyList>,
    mut bones: Query<&mut BoneMotor>,
) {
    for body in bodies.bodies.iter_mut() {
        for muscle in body.angular_muscles.iter_mut() {
            muscle.contract(&mut bones, f32::sin(time.elapsed_seconds() * 0.1) * 200.0);
        }
    }
}
