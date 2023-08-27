use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{bone::BoneMotor, joint::Joint};

pub struct Muscle {
    pub bone_motors: [Entity; 2],
}
impl Muscle {
    pub fn contract(&mut self, motors: &mut Query<&mut BoneMotor>, impulse: f32) {
        println!("contracting muscle, {:?}", impulse);
        let [mut a, mut b] = motors.get_many_mut(self.bone_motors).unwrap();
        a.apply_impulse(impulse * 0.5);
        b.apply_impulse(-impulse * 0.5);
    }
}
