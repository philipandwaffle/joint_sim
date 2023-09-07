use std::f32::consts::PI;

use bevy::{
    math::vec2,
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, ComputedVisibility, Entity,
        GlobalTransform, Quat, SpatialBundle, Transform, Vec2, Vec3,
    },
    transform::TransformBundle,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};
use bevy_rapier2d::prelude::{
    Collider, ImpulseJoint, LockedAxes, RevoluteJointBuilder, RigidBody, Sensor,
};

use super::joint::JointBundle;

#[derive(Bundle)]
pub struct BoneBundle {}

pub struct Bone;
impl Bone {
    // Development fn for testing new bone
    pub fn new(commands: &mut Commands, joints: [Entity; 2], joint_pos: [Vec2; 2]) -> Entity {
        let [a_pos, b_pos] = joint_pos;

        // Create joint
        let ab = b_pos - a_pos;
        let dir = ab * 0.5;
        let len = ab.length();
        let x = if ab.x >= 0.0 { -1.0 } else { 1.0 };
        let z_rot = x * f32::acos(ab.y / len);
        let bone_width = 3.0;

        let bone_rect = shapes::Rectangle {
            extents: vec2(bone_width, len),
            origin: shapes::RectangleOrigin::Center,
        };

        let bone_ent = commands
            .spawn((
                RigidBody::Dynamic,
                SpatialBundle::from_transform(Transform::from_translation(
                    (a_pos + dir).extend(-0.1),
                )),
            ))
            .with_children(|p| {
                p.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&bone_rect),
                        transform: Transform::from_rotation(Quat::from_rotation_z(z_rot)),
                        ..default()
                    },
                    Fill::color(Color::hsl(360.0, 0.37, 0.84)),
                    Collider::cuboid(bone_width * 0.5, (len - 10.0) * 0.5),
                    Sensor,
                ));
            })
            .id();

        let bearing_a = RevoluteJointBuilder::new().local_anchor1(-dir).build();
        let bearing_b = RevoluteJointBuilder::new().local_anchor1(dir).build();
        let axel_a = commands.spawn(ImpulseJoint::new(bone_ent, bearing_a)).id();
        let axel_b = commands.spawn(ImpulseJoint::new(bone_ent, bearing_b)).id();

        commands.get_entity(joints[0]).unwrap().add_child(axel_a);
        commands.get_entity(joints[1]).unwrap().add_child(axel_b);
        return bone_ent;
    }
}
