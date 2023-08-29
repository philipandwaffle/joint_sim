use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct JointBundle {
    joint: Joint,
    rigid_body: RigidBody,
    mass: AdditionalMassProperties,
    external_impulse: ExternalImpulse,
    damping: Damping,
    shape_bundle: ShapeBundle,
    fill: Fill,
    stroke: Stroke,
    collider: Collider,
    gravity: GravityScale,
    ccd: Ccd,
}
impl JointBundle {
    pub fn new(
        translation: Vec2,
        radius: f32,
        linear_damping: f32,
        angular_damping: f32,
    ) -> JointBundle {
        let circle = shapes::RegularPolygon {
            sides: 16,
            feature: shapes::RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        };

        return JointBundle {
            damping: Damping {
                linear_damping,
                angular_damping,
            },
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&circle),
                transform: Transform::from_translation(translation.extend(0.0)),
                ..default()
            },
            collider: Collider::ball(radius),
            ..default()
        };
    }
    pub fn from_translation(translation: Vec2) -> Self {
        let mut joint = JointBundle::default();
        joint.shape_bundle.transform = Transform::from_translation(translation.extend(0.0));
        return joint;
    }
}
impl Default for JointBundle {
    fn default() -> Self {
        let linear_damping = 0.5;
        let angular_damping = 0.5;
        let radius = 10.0;
        let circle = shapes::RegularPolygon {
            sides: 16,
            feature: shapes::RegularPolygonFeature::Radius(radius),
            ..default()
        };

        return JointBundle {
            joint: Joint,
            rigid_body: RigidBody::Dynamic,
            mass: AdditionalMassProperties::Mass(20.0),
            external_impulse: ExternalImpulse::default(),
            damping: Damping {
                linear_damping,
                angular_damping,
            },
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&circle),
                transform: Transform::default(),
                ..default()
            },
            fill: Fill::color(Color::hsl(108.0, 0.83, 0.33)),
            stroke: Stroke::new(Color::hsl(108.0, 0.89, 0.14), 0.0),
            collider: Collider::ball(radius),
            gravity: GravityScale::default(),
            ccd: Ccd::default(),
        };
    }
}

#[derive(Component)]
pub struct Joint;
