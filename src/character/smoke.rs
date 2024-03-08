use bevy::prelude::*;

use crate::script::systems::moving::*;


#[derive(Component)]
pub struct Smoke{
    pub motion: Motion,
    pub path: Path,
}

impl Smoke{
    pub fn new() -> Self{
        Smoke{
            motion: Motion::default(),
            path: Path::default(),
        }
    }
}

pub fn smoke_diffuse(
    mut query: Query<(&mut Transform, &Smoke)>,
    time: Res<Time>,
){
    for (mut tranform, smoke) in query.iter_mut() {
        tranform.translation += smoke.motion.direction * smoke.motion.speed*time.delta_seconds();
    }
}