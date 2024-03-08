use bevy::prelude::*;

#[derive(Resource)]
pub struct Electricity {
    pub voltage: f32
}

impl Default for Electricity {
    fn default() -> Self {
        Electricity {
            voltage: 0.0
        }
    }
}

impl Electricity {
    pub fn power_on(&mut self){
        self.voltage = 220.0;
    }

    pub fn power_off(&mut self){
        self.voltage = 0.0;
    }
}

pub fn set_electricity(
    mut elect: ResMut<Electricity>,
){
    elect.power_on();
}

pub fn cut_electricity(
    mut elect: ResMut<Electricity>,
){
    elect.power_off();
}


