use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct BodyList {
    pub bodies: Vec<Body>,
}

#[derive(Resource)]
pub struct Body {
    pub muscles: Vec<[Entity; 2]>,
}
