use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{bone::BoneMotor, joint::Joint};

pub struct AngularMuscle {
    pub bone_motors: [Entity; 2],
}
impl AngularMuscle {
    pub fn contract(&mut self, motors: &mut Query<&mut BoneMotor>, impulse: f32) {
        println!("contracting muscle, {:?}", impulse);
        let [mut a, mut b] = motors.get_many_mut(self.bone_motors).unwrap();
        a.apply_impulse(impulse * 0.5);
        b.apply_impulse(-impulse * 0.5);
    }
}

#[derive(Bundle)]
pub struct LinearMuscleBundle {
    linear_muscle: LinearMuscleMotor,
}
impl LinearMuscleBundle {
    pub fn new(commands: &mut Commands, joints: [Entity; 2]) -> Entity {
        let joint_ab = PrismaticJointBuilder::new(Vec2::X)
            .motor_velocity(50.0, 1.0)
            .local_anchor1(Vec2::ZERO)
            .local_anchor2(Vec2::ZERO)
            .build();

        let impulse_joint = commands
            .spawn((
                ImpulseJoint::new(joints[0], joint_ab),
                LinearMuscleMotor::default(),
            ))
            .id();

        commands
            .get_entity(joints[1])
            .unwrap()
            .add_child(impulse_joint);

        return impulse_joint;
    }
}

#[derive(Component)]
pub struct LinearMuscleMotor {
    pub target_vel: f32,
}
impl LinearMuscleMotor {
    pub fn apply_delta_velocity(&mut self, delta_vel: f32) {
        self.target_vel += delta_vel;
    }
}
impl Default for LinearMuscleMotor {
    fn default() -> Self {
        Self { target_vel: 0.0 }
    }
}

pub fn apply_linear_velocity_delta(mut muscles: Query<(&LinearMuscleMotor, &mut ImpulseJoint)>) {
    for (m, ij) in muscles.iter_mut() {
        println!("applying impulse, {:?}", m.target_vel);
        let revolute_joint = *ij.data.as_prismatic().unwrap();
        let mut motor = *revolute_joint.motor().unwrap();
        motor.target_vel = m.target_vel;
    }
}
