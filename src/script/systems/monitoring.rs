use bevy::prelude::*;

use crate::character::{smoke::*, fire::*};

#[derive(Component)]
pub struct SmokeDetector{
    pub id: usize,
    pub density: f32,
    pub location: Vec3,
    pub duration: f32,
}

#[derive(Component)]
pub struct TemperatureSensor{
    pub id: usize,
    pub temperature: f32,
    pub location: Vec3,
    pub duration: f32,
}

impl Default for SmokeDetector {
    fn default() -> Self {
        SmokeDetector {
            id: 0,
            density: 0.0,
            location: Vec3::new(0.0, 0.0, 0.0),
            duration: 0.0,
        }
    }
}

impl Default for TemperatureSensor {
    fn default() -> Self {
        TemperatureSensor {
            id: 0,
            temperature: 0.0,
            location: Vec3::new(0.0, 0.0, 0.0),
            duration: 0.0,
        }
    }
}

pub fn detect_smoke(
    smoke: Query<&Transform, With<Smoke>>,
    mut smoke_detector: Query<(&Transform, &mut SmokeDetector)>
){
    for (detector_transform,mut detector) in smoke_detector.iter_mut() {
        for smoke_transform in smoke.iter() {
            let distance = detector_transform.translation.distance(smoke_transform.translation);
            if distance < 1.0 {
                detector.density += 1.0;
            }
        }
    }
}

pub fn detect_temperature(
    temperature: Query<&Transform, With<Fire>>,
    mut temperature_sensor: Query<(&Transform, &mut TemperatureSensor)>
){
    for (sensor_transform,mut sensor) in temperature_sensor.iter_mut() {
        for temperature_transform in temperature.iter() {
            let distance = sensor_transform.translation.distance(temperature_transform.translation);
            if distance < 1.0 {
                sensor.temperature += 1.0;
            }
        }
    }
}