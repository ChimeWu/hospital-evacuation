use bevy::prelude::*;

#[derive(Resuorce)]
pub struct Electricity {
    pub voltage: f32,
    pub current: f32,
    pub resistance: f32,
    pub power: f32,
    pub energy: f32,
    pub temperature: f32,
}