use bevy::prelude::*;

#[derive(Component)]
pub struct SecurityPosture {
    pub level: u8,
    pub status: SecurityStatus,
}

#[derive(Component)]
pub enum SecurityStatus {
    Safe,
    Warning,
    Alert,
    Emergency,
}

#[derive(Component)]
pub struct SecurityState {
    pub posture: SecurityPosture,
    pub messages: Vec<Message>,
}

pub trait Monitor {
    fn get_message(&self, key: &str) -> Option<&str>;
    fn set_message(&mut self, key: &str, value: &str);
}