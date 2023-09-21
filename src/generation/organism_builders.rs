use bevy::math::vec2;

use crate::organism::organism::OrganismBuilder;

pub fn get_mem_leak_test() -> OrganismBuilder {
    let brain_structure = vec![10, 10, 10];
    let joint_pos = vec![
        vec2(60.0, 0.0),
        vec2(20.0, 0.0),
        vec2(0.0, 0.0),
        vec2(-20.0, 0.0),
        vec2(-60.0, 0.0),
        vec2(60.0, 40.0),
        vec2(20.0, 40.0),
        vec2(0.0, 40.0),
        vec2(-20.0, 40.0),
        vec2(-60.0, 40.0),
        vec2(60.0, 80.0),
        vec2(20.0, 80.0),
        vec2(0.0, 80.0),
        vec2(-20.0, 80.0),
        vec2(-60.0, 80.0),
    ];

    // let bones = vec![[0,1],[1,2],[3,4],[4,5],[5,6]];
    let bones = vec![[0, 1]];
    // let bones = vec![];
    // let muscles = vec![[3, 2], [4, 0], [5, 1], [6, 2]];
    let muscles = vec![];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

pub fn get_runner_v6() -> OrganismBuilder {
    let brain_structure = vec![16, 16, 16];
    let joint_pos = vec![
        vec2(-40.0, 0.0),
        vec2(40.0, 0.0),
        vec2(0.0, 30.0),
        vec2(-30.0, 40.0),
        vec2(30.0, 40.0),
    ];

    let bones = vec![[0, 3], [1, 4], [2, 3], [2, 4], [3, 4]];
    // let bones = vec![];
    let muscles = vec![[0, 2], [1, 3]];
    // let muscles = vec![];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

pub fn get_runner_v5() -> OrganismBuilder {
    let brain_structure = vec![16, 16, 16];
    let joint_pos = vec![
        vec2(-30.0, 0.0),
        vec2(-40.0, 30.0),
        vec2(-30.0, 60.0),
        vec2(-20.0, 30.0),
        vec2(30.0, 0.0),
        vec2(40.0, 30.0),
        vec2(30.0, 60.0),
        vec2(20.0, 30.0),
        vec2(0.0, 20.0),
        vec2(0.0, 40.0),
    ];

    let bones = vec![
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        [1, 3],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        [5, 7],
        [3, 9],
        [9, 7],
        [7, 8],
        [8, 3],
        [8, 9],
    ];
    // let bones = vec![];
    let muscles = vec![[2, 10], [3, 13], [7, 11], [8, 12]];
    // let muscles = vec![];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

pub fn get_runner_v4() -> OrganismBuilder {
    let brain_structure = vec![16, 16, 16];
    let joint_pos = vec![
        vec2(-30.0, 0.0),
        vec2(30.0, 0.0),
        vec2(0.0, 30.0),
        vec2(-30.0, 60.0),
        vec2(30.0, 60.0),
    ];

    let bones = vec![[0, 1], [1, 2], [2, 0], [2, 4], [4, 3], [3, 2]];
    // let bones = vec![];
    let muscles = vec![[1, 3], [2, 5]];
    // let muscles = vec![];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

pub fn get_runner_v3() -> OrganismBuilder {
    let brain_structure = vec![16, 16, 16];
    let joint_pos = vec![
        vec2(-30.0, 0.0),
        vec2(30.0, 0.0),
        vec2(-40.0, 40.0),
        vec2(0.0, 40.0),
        vec2(40.0, 40.0),
        vec2(-20.0, 60.0),
        vec2(20.0, 60.0),
    ];

    let bones = vec![
        [0, 2],
        [2, 3],
        [3, 0],
        [1, 3],
        [3, 4],
        [4, 1],
        [3, 5],
        [5, 6],
        [6, 3],
    ];
    // let bones = vec![];
    let muscles = vec![[1, 6], [4, 8], [2, 3]];
    // let muscles = vec![];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

pub fn get_runner_v2() -> OrganismBuilder {
    let brain_structure = vec![10, 10, 10];
    let joint_pos = vec![
        vec2(0.0, 65.0),
        vec2(-45.0, 40.0),
        vec2(45.0, 40.0),
        vec2(-45.0, 0.0),
        vec2(-15.0, 10.0),
        vec2(15.0, 10.0),
        vec2(45.0, 0.0),
    ];

    let bones = vec![[1, 0], [0, 2], [2, 1], [3, 1], [4, 0], [5, 0], [6, 2]];
    // let bones = vec![];
    let muscles = vec![[3, 2], [4, 0], [5, 1], [6, 2]];
    // let muscles = vec![];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}

pub fn muscle_test_organism() -> OrganismBuilder {
    let joint_pos = vec![vec2(0.0, 0.0), vec2(25.0, 50.0), vec2(50.0, 0.0)];
    let bones = vec![[1, 2], [0, 1]];
    let muscles = vec![[1, 0]];

    return OrganismBuilder::new(1, vec![3, 3], joint_pos, bones, muscles);
}

pub fn bone_test_organism() -> OrganismBuilder {
    let brain_structure = vec![2, 2];

    let dx = 40.0;
    let dy0 = 40.0;
    let dy1 = 80.0;
    let dy2 = 120.0;
    let dy3 = 160.0;
    let joint_pos = vec![
        vec2(-dx, dy0),
        vec2(dx, dy0),
        vec2(0.0, dy1),
        vec2(0.0, dy2),
        vec2(-dx, dy3),
        vec2(dx, dy3),
    ];
    // let bones = vec![[0, 1], [1, 2], [2, 0]];
    let bones = vec![[0, 1], [2, 0], [2, 1], [2, 3], [3, 4], [4, 5], [5, 3]];
    let muscles = vec![];
    let ob = OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
    return ob;
}

pub fn get_runner_builder() -> OrganismBuilder {
    let brain_structure = vec![6, 6];
    let joint_pos = vec![
        vec2(-20.0, 80.0),
        vec2(20.0, 80.0),
        vec2(-40.0, 60.0),
        vec2(0.0, 60.0),
        vec2(40.0, 60.0),
        vec2(-40.0, 25.0),
        vec2(40.0, 25.0),
    ];

    let bones = vec![
        [0, 1],
        [2, 0],
        [0, 3],
        [1, 3],
        [4, 1],
        [5, 0],
        [6, 1],
        [3, 2],
        [3, 4],
    ];
    let muscles = vec![[5, 0], [6, 0]];

    return OrganismBuilder::new(1, brain_structure, joint_pos, bones, muscles);
}
