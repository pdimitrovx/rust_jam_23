use std::{fs, path::Path};

use macroquad::prelude::*;
use once_cell::sync::OnceCell;

//shamelessly ripped off
pub static RESOURCES: OnceCell<Resources> = OnceCell::new();

const AIR_OBSTACLE_PATH: &str = "./res/obstacles_air/";
const GROUND_OBSTACLE_PATH: &str = "./res/obstacles_ground/";
pub struct Resources {
    pub ground_obstacles: Vec<Texture2D>,
    pub air_obstacles: Vec<Texture2D>,
    pub santa_texture: Texture2D,
    pub background_houses_texture: Texture2D,
    pub background_trees_texture: Texture2D,
    pub background_texture: Texture2D,
    pub play_button_spritesheet_texture: Texture2D,
    pub quit_button_spritesheet_texture: Texture2D,
}

pub async fn init_resources() {
    let resources = Resources::new().await;
    match RESOURCES.set(resources) {
        Ok(_) => println!("Resources init successfull"),
        Err(_) => panic!("Failed to load Resources"),
    };
}

impl Resources {
    pub async fn new() -> Self {
        let ground_obstacles: Vec<Texture2D> = Vec::new();
        let air_obstacles: Vec<Texture2D> = Vec::new();

        let mut resources = Self {
            ground_obstacles: ground_obstacles,
            air_obstacles: air_obstacles,
            santa_texture: load_texture("res/santa.png").await.unwrap(),
            background_texture: load_texture("res/background/background-houses-road-590x1800.png").await.unwrap(),
            background_houses_texture: load_texture("res/background/background-houses-road-270x1800.png")
                .await
                .unwrap(),
            background_trees_texture: load_texture("res/background/background-snowy-trees-270x1800.png")
                .await
                .unwrap(),
            play_button_spritesheet_texture: load_texture("res/play_button.png").await.unwrap(),
            quit_button_spritesheet_texture: load_texture("res/quit_button.png").await.unwrap(),
        };

        let ground_obstacle_dir_path = Path::new(GROUND_OBSTACLE_PATH);
        let air_obstacle_dir_path = Path::new(AIR_OBSTACLE_PATH);

        if ground_obstacle_dir_path.is_dir() {
            if let Ok(entries) = fs::read_dir(ground_obstacle_dir_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path().to_string_lossy().to_string();

                        if let Ok(thing) = load_texture(&file_path).await {
                            resources.ground_obstacles.push(thing);
                        }
                    }
                }
            }
        }

        if air_obstacle_dir_path.is_dir() {
            if let Ok(entries) = fs::read_dir(air_obstacle_dir_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path().to_string_lossy().to_string();

                        if let Ok(thing) = load_texture(&file_path).await {
                            resources.air_obstacles.push(thing);
                        }
                    }
                }
            }
        }
        //todo add panic here if fail

        resources
    }

    pub fn get_random_ground_obstacle(&self) -> Texture2D {
        let len = self.ground_obstacles.len();
        if len > 0 { 
            let index = macroquad::rand::RandomRange::gen_range(0,len);
            self.ground_obstacles[index].clone()
        } else {
            self.ground_obstacles[0].clone()
        }
    }

    pub fn get_random_air_obstacle(&self) -> Texture2D {
        let len = self.air_obstacles.len();
        if len > 0 { 
            let index = macroquad::rand::RandomRange::gen_range(0,len);
            self.air_obstacles[index].clone()
        } else {
            self.air_obstacles[0].clone()
        }
    }
}
