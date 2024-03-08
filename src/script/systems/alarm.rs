use bevy::prelude::*;

#[derive(Component)]
pub enum SecurityStatus {
    Safe,
    Warning(Time),
    Alert,
    Emergency,
}

#[derive(Component)]
pub struct FireAlarm {
    pub id: usize,
    pub status: SecurityStatus,
    pub location: Vec3,
    pub duration: f32,
}