use bevy::prelude::*;
use rand::Rng;

pub mod file_process;
pub mod map_constants;

use file_process::*;
use map_constants::*;
use super::stuffs::*;

use std::fs;
use std::io::Error;



#[derive(Resource)]
pub struct Map{
    data: Vec<Vec<char>>,
    wind_size: (f32, f32),
    grid_size: (usize, usize),
    elem_size: f32,
    location: (Vec3, Vec3, Vec3, Vec3),
 
}

impl Map{
    pub fn new() -> Self{
        Map{
            data: vec![vec!['_'; MAP_COLUMNS]; MAP_ROWS],
            wind_size: (0.0, 0.0),
            grid_size: (0, 0),
            elem_size: 0.0,
            location: (Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn map_load_txt(&mut self,path: &str) -> Result<(),Error> {
        let file_content = fs::read_to_string(path)?;
        self.data = file_content
            .lines()
            .map(|line| line.split_whitespace().map(|c| c.chars().next().unwrap()).collect())
            .collect::<Vec<Vec<char>>>();
        Ok(())
    }

    pub fn map_write_txt(&self, path: &str){
        let arr_string = self.data.iter()
        .map(|row| row.iter().map(|&i| i.to_string()).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");
    
        fs::write(path, arr_string).expect("Unable to write file");        
    }
}

fn load_map_data(
    mut map: ResMut<Map>,
    window: Query<&Window>
){

    let window = window.single();
    map.wind_size = (window.width(), window.height());
    map.grid_size = (MAP_ROWS, MAP_COLUMNS);
    map.elem_size = window.height() / (MAP_ROWS as f32);
    map.location = (
        Vec3::new(window.width()/2.0 - MAP_COLUMNS as f32*map.elem_size, -window.height()/2.0, 0.0),
        Vec3::new(window.width()/2.0 - MAP_COLUMNS as f32*map.elem_size, window.height()/2.0, 0.0),
        Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
        Vec3::new(window.width()/2.0, -window.height()/2.0, 0.0),
    );
}

fn change_tile_texture(

){

}

fn spawn_map(
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut commands: Commands,
){
    let start_point = map.location.0;
    let element_size = map.elem_size;
    let (rows, columns) = map.grid_size;

    let get_x = |i,j|{
        start_point.x + element_size/2.0 + (j as f32) * element_size
    };
    
    let get_y = |i, j| {
        start_point.y - element_size/2.0 + (i as f32) * element_size
    };

    for i in 0..rows {
        for j in 0..columns{
            let x = get_x(i, j);
            let y = get_y(i, j);
            match map.data[i][j] {
                'W' => {
                    commands.spawn((
                        SpriteBundle{
                            texture: asset_server.load("./texture/wall.png"),
                            sprite:Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        },
                        WallTile,
                    )
                    );
                }
                '_' => {
                    commands.spawn((
                        SpriteBundle{
                            texture: asset_server.load("./texture/floor.png"),
                            sprite:Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        },
                        FloorTile,
                    )
                    );
                }
                'D' => {
                    commands.spawn((
                        SpriteBundle{
                            texture: asset_server.load("./texture/door.png"),
                            sprite:Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        },
                        DoorTile,
                    )
                    );
                }
                'E' => {
                    commands.spawn((
                        SpriteBundle{
                            texture: asset_server.load("./texture/exit.png"),
                            sprite:Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        },
                        ExitTile,
                    )
                    );
                }
                'F' => {
                    commands.spawn((
                        SpriteBundle{
                            texture: asset_server.load("./texture/furniture.png"),
                            sprite:Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        },
                        Furniture,
                    )
                    );
                }
                _ => {}
            }

            let mut bool1 = rand::thread_rng().gen_bool(0.05);
            if bool1 {
                commands.spawn((
                    SpriteBundle{
                        texture: asset_server.load("./texture/human.png"),
                        sprite:Sprite {
                            custom_size: Some(Vec2::new(element_size, element_size)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                        ..Default::default()
                    },
                    Human,
                )
                );
            }


        }
    }
}
