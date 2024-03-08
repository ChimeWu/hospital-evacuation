use bevy::prelude::*;

use super::smoke::*;

#[derive(Component)]
pub struct Fire;

#[derive(Component)]
pub struct Tinder{
    pub id: usize,
    pub position: Vec3,
    pub duration: f32,
    pub kind: TinderKind,
    pub state: FireState,  
}

#[derive(Component)]
pub enum TinderKind{
    Wood,
    Paper,
    Cloth,
    Gas,
    Oil,
    Plastic,
    Other,
}

#[derive(Component, Default)]
pub enum FireState{
    #[default]
    NotBurn,
    Burning(FireTimer),
    Extinguished,
}

#[derive(Component)]
pub struct FireTimer{
    pub fire: Timer,
    pub smoke: Timer,
}

impl Tinder {
    pub fn caught_fire(&mut self){
        self.state = FireState::Burning(FireTimer{
            fire: Timer::from_seconds(60.0, TimerMode::Once),
            smoke: Timer::from_seconds(5.0, TimerMode::Repeating),
        });
    }

    pub fn burning(&mut self, time: Res<Time>){
        match self.state{
            FireState::Burning(ref mut timer) => {
                timer.fire.tick(time.delta());
                timer.smoke.tick(time.delta());
                if timer.fire.finished(){
                    self.state = FireState::Extinguished;
                }
            },
            _ => {},
        }
    }

    pub fn make_smoke(&self, commands: &mut Commands, assets_server: Res<AssetServer>){
        match self.state{
            FireState::Burning(ref timer) => {
                if timer.smoke.just_finished(){
                    commands.spawn((
                        SpriteBundle{
                            texture: assets_server.load("./texture/smoke.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(10.0, 10.0)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(self.position),
                            ..Default::default()
                        },
                        Smoke::new(),
                    )
                    );
                }
            },
            _ =>{},
        };
    }
}





