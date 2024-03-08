use bevy::prelude::*;


#[derive(Component)]
pub struct Information {
    pub messages: Vec<Message>,
}

#[derive(Component)]
pub struct Message {
    pub sender: String,
    pub receiver: String,
    pub content: String,
}


pub trait Correspond {
    fn get_message(&self, key: &str) -> Option<&str>;
    fn set_message(&mut self, key: &str, value: &str);
}