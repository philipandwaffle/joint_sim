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

        let impulse_joint = commands.spawn(ImpulseJoint::new(joints[0], joint_ab)).id();

        commands
            .get_entity(joints[1])
            .unwrap()
            .add_child(impulse_joint);

        return impulse_joint;
    }
}
