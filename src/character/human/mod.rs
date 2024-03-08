use bevy::prelude::*;

use crate::script::systems::moving::*;

#[derive(Component)]
pub struct Human {
    pub id: usize,
    pub physical_state: PhysicalState,
    pub mental_state: MentalState,
    pub motion: Motion,
    pub path: Path,
}

#[derive(Component)]
pub struct Health {
    pub value: u8,
    pub max_value: u8,
}

#[derive(Component)]
pub struct Strength {
    pub value: u8,
    pub max_value: u8,
}

#[derive(Component)]
pub enum Gender{
    Man,
    Woman,
}

#[derive(Component)]
pub struct PhysicalState {
    pub health: Health,
    pub strength: Strength,
    pub sex: Gender,
    pub age: u8,
    pub bmi: f32,
}

#[derive(Component)]
pub struct MentalState {
    pub stress: u8,
    pub happiness: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub sanity: u8,
}



#[derive(Component)]
pub struct SocialState {
    pub profession: Profession,
    pub level: u8,
    pub salary: f32,
    pub education: u8,
    pub experience: u8,
    pub wealth: f32,
    pub friends: Vec<usize>,
    pub enemies: Vec<usize>,
    pub family: Vec<usize>,
    pub colleagues: Vec<usize>,
}

#[derive(Component)]
pub enum Profession {
    Doctor,
    Nurse,
    Firefighter,
    Police,
    Teacher,
    Student,
    Unemployed,
}
                                                                                                                            

