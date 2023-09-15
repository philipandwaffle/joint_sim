use bevy::{
    prelude::{default, Bundle, Component, Handle, Transform, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, Ccd, Collider, ColliderMassProperties, Damping, ExternalImpulse,
    Friction, GravityScale, LockedAxes, RigidBody,
};

// Bundle for spawning an organisms joint
#[derive(Bundle)]
pub struct JointBundle {
    joint: Joint,
    rigid_body: RigidBody,
    mass: AdditionalMassProperties,
    collider_mass: ColliderMassProperties,
    external_impulse: ExternalImpulse,
    damping: Damping,
    friction: Friction,
    collider: Collider,
    // collision_layer: CollisionLayer,
    // active_hooks: ActiveHooks,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    gravity: GravityScale,
    ccd: Ccd,
    locked_axis: LockedAxes,
}
impl JointBundle {
    pub fn from_translation(
        translation: Vec2,
        mesh: &Mesh2dHandle,
        material: &Handle<ColorMaterial>,
    ) -> Self {
        let starting_damping = 10000000.0;
        let radius = 5.0;
        let mass = 0.5;

        return Self {
            joint: Joint,
            rigid_body: RigidBody::Dynamic,
            mass: AdditionalMassProperties::Mass(mass),
            collider_mass: ColliderMassProperties::Density(0.2),
            external_impulse: ExternalImpulse::default(),
            damping: Damping {
                linear_damping: starting_damping,
                angular_damping: 0.0,
            },
            friction: Friction::coefficient(0.7),
            collider: Collider::ball(radius),
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(translation.extend(0.0)),
                ..default()
            },
            gravity: GravityScale(5.0),
            ccd: Ccd::default(),
            locked_axis: LockedAxes::ROTATION_LOCKED,
        };
    }
}

// Component that marks entities as joints
#[derive(Component)]
pub struct Joint;
