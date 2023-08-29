use bevy::{math::vec2, prelude::*};

use crate::organism::{
    bone::Bone,
    joint::JointBundle,
    muscle::Muscle,
    organism::{Organism, OrganismList},
};

pub struct OrganismTestingPlugin;
impl Plugin for OrganismTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_running_organism);
        // app.add_systems(Startup, spawn_organism_test);
    }
}

fn spawn_running_organism(mut commands: Commands) {
    let a_pos = vec2(-80.0, 70.0);
    let b_pos = vec2(80.0, 70.0);
    let c_pos = vec2(-100.0, 60.0);
    let d_pos = vec2(0.0, 50.0);
    let e_pos = vec2(100.0, 60.0);
    let f_pos = vec2(-70.0, 30.0);
    let g_pos = vec2(70.0, 30.0);
    let h_pos = vec2(90.0, 20.0);
    let i_pos = vec2(-90.0, 20.0);

    let a_ent = commands.spawn(JointBundle::from_translation(a_pos)).id();
    let b_ent = commands.spawn(JointBundle::from_translation(b_pos)).id();
    let c_ent = commands.spawn(JointBundle::from_translation(c_pos)).id();
    let d_ent = commands.spawn(JointBundle::from_translation(d_pos)).id();
    let e_ent = commands.spawn(JointBundle::from_translation(e_pos)).id();
    let f_ent = commands.spawn(JointBundle::from_translation(f_pos)).id();
    let g_ent = commands.spawn(JointBundle::from_translation(g_pos)).id();
    let h_ent = commands.spawn(JointBundle::from_translation(h_pos)).id();
    let i_ent = commands.spawn(JointBundle::from_translation(i_pos)).id();

    Bone::new(&mut commands, [a_ent, b_ent], [a_pos, b_pos], None);
    Bone::new(&mut commands, [a_ent, c_ent], [a_pos, c_pos], None);
    Bone::new(&mut commands, [a_ent, d_ent], [a_pos, d_pos], None);
    Bone::new(&mut commands, [b_ent, d_ent], [b_pos, d_pos], None);
    Bone::new(&mut commands, [b_ent, e_ent], [b_pos, e_pos], None);
    Bone::new(&mut commands, [c_ent, f_ent], [c_pos, f_pos], None);
    Bone::new(&mut commands, [e_ent, g_ent], [e_pos, g_pos], None);
    Bone::new(&mut commands, [f_ent, h_ent], [f_pos, h_pos], None);
    Bone::new(&mut commands, [g_ent, i_ent], [g_pos, i_pos], None);

    let muscles = vec![
        Muscle::new([c_ent, h_ent]),
        Muscle::new([d_ent, f_ent]),
        Muscle::new([d_ent, g_ent]),
        Muscle::new([e_ent, i_ent]),
    ];

    let body = Organism::new(vec![4, 6, 6, 6, 4], muscles);
    commands.insert_resource(OrganismList {
        organisms: vec![body],
    })
}

fn spawn_organism_test(mut commands: Commands) {
    let a_pos = vec2(-100.0, 50.0);
    let b_pos = vec2(0.0, 150.0);
    let c_pos = vec2(100.0, 50.0);
    let d_pos = vec2(-150.0, 100.0);
    let e_pos = vec2(150.0, 100.0);

    let a_ent = commands.spawn(JointBundle::from_translation(a_pos)).id();
    let b_ent = commands.spawn(JointBundle::from_translation(b_pos)).id();
    let c_ent = commands.spawn(JointBundle::from_translation(c_pos)).id();
    let d_ent = commands.spawn(JointBundle::from_translation(d_pos)).id();
    let e_ent = commands.spawn(JointBundle::from_translation(e_pos)).id();

    Bone::new(&mut commands, [a_ent, b_ent], [a_pos, b_pos], None);
    Bone::new(&mut commands, [b_ent, c_ent], [b_pos, c_pos], None);
    Bone::new(&mut commands, [c_ent, a_ent], [c_pos, a_pos], None);
    Bone::new(&mut commands, [b_ent, d_ent], [b_pos, d_pos], None);
    Bone::new(&mut commands, [b_ent, e_ent], [b_pos, e_pos], None);

    let body = Organism::new(
        vec![2, 4, 2],
        vec![Muscle::new([a_ent, d_ent]), Muscle::new([c_ent, e_ent])],
    );
    commands.insert_resource(OrganismList {
        organisms: vec![body],
    });
}
