use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Bone;
impl Bone {
    pub fn new(
        commands: &mut Commands,
        joints: [Entity; 2],
        joint_pos: [Vec2; 2],
        length: Option<f32>,
    ) -> Entity {
        let [mut a_pos, mut b_pos] = joint_pos;

        if let Some(length) = length {
            let dir = b_pos - a_pos;
            let len_diff = dir.length() - length;
            let correction_vec = dir.normalize() * len_diff * 0.5;
            a_pos += correction_vec;
            b_pos -= correction_vec;
        }

        let joint_ab = RevoluteJointBuilder::new()
            .motor_velocity(-1.0, 0.5)
            // .motor_max_force(10.0)
            // .motor(0.0, 0.0, 0.5, 0.5)
            // .motor_velocity(1.0, 1.0)
            .local_anchor1(a_pos - a_pos)
            .local_anchor2(b_pos - a_pos)
            .build();

        let impulse_joint = commands
            .spawn((ImpulseJoint::new(joints[0], joint_ab), BoneMotor::default()))
            .id();

        commands
            .get_entity(joints[1])
            .unwrap()
            .add_child(impulse_joint);

        return impulse_joint;
    }
}

#[derive(Component)]
pub struct BoneMotor {
    target_impulse: f32,
}
impl BoneMotor {
    pub fn apply_impulse(&mut self, delta_impulse: f32) {
        self.target_impulse += delta_impulse;
    }
}
impl Default for BoneMotor {
    fn default() -> Self {
        Self {
            target_impulse: 0.0,
        }
    }
}

pub fn apply_motor_impulse(mut motors: Query<(&BoneMotor, &mut ImpulseJoint)>) {
    for (b, ij) in motors.iter_mut() {
        println!("applying impulse, {:?}", b.target_impulse);
        let revolute_joint = *ij.data.as_revolute().unwrap();
        let mut motor = *revolute_joint.motor().unwrap();
        motor.target_vel = b.target_impulse;
    }
}
