use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct Motion {
    pub direction: Vec3,
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub max_acceleration: f32,
}

#[derive(Component)]
pub struct Path{
    pub calculated: Vec<Vec3>,
    pub traveld: Vec<Vec3>,
    pub position_now: Vec3,
    pub position_next: Vec3,
    pub destination: Vec3,
    pub destination_reached: bool,
    pub mileage: f32,
}


#[derive(Component)]
pub struct Mass(f32);

#[derive(Component)]
pub struct Force(Vec3);

#[derive(Component)]
pub struct Momentum(Vec3);

#[derive(Component)]
pub struct Energy(f32);

#[derive(Component)]
pub struct Temperature(f32);