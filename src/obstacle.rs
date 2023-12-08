use macroquad::prelude::*;
use macroquad::{math::Vec2, texture::Texture2D, color::WHITE};

use crate::resources::RESOURCES;

pub const BACKGROUND_SPEED: f32 = 1.5;
pub const DEFAULT_OBSTACLE_SPEED: f32 = 2.0;
pub const OBSTACLE_WIDTH_HOUSE: i32 = 64;
pub const OBSTACLE_HEIGHT_HOUSE: i32 = 35; //
pub const STARTING_NUMBER_OF_OBSTACLES: usize = 2;
pub const MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES: f32 = 64.0;

#[derive(Clone)]
pub struct Obstacle {
    pub position: Vec2,
    //example has height, maybe we need height as optional on houses?
    pub speed: f32,
    pub texture: Texture2D,
    pub rect: Rect,
}

impl Obstacle {
    pub fn new(position: Vec2, speed: Option<f32>, texture: Texture2D) -> Obstacle {
        // pub  fn new(texture_filepath: &str ) -> Obstacle {
        Obstacle {
            position: position,
            speed: speed.unwrap_or(DEFAULT_OBSTACLE_SPEED),
            texture: texture,
            // texture: vec![load_texture(texture_filepath).unwrap()],
            rect: Rect::new(0., 0., OBSTACLE_WIDTH_HOUSE as f32, OBSTACLE_HEIGHT_HOUSE as f32),
        }
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }

    fn update(&mut self) {
        self.position.x -= self.speed;
    }
}

pub struct ObstacleManager {
    obstacles: Vec<Obstacle>,
    // increment for each obstacle that goes past, ie score
    number_of_cleared: u32,
}
impl ObstacleManager {
    pub fn new() -> Self {
        let mut x_pos: f32 = screen_width() / 2.0; //Some screen value to spawn initial ground obstacles;
        let mut manager = Self {
            obstacles: Vec::new(),
            number_of_cleared: 0,
        };

        //TODO: random (house/ground obst) texture?
        let resources = RESOURCES.get().unwrap();
        // let texture = &resources.house_texture;

        for _ in 0..STARTING_NUMBER_OF_OBSTACLES {
            manager.add_obstacle(x_pos - 10.0, resources.house_texture.clone());
            x_pos -= OBSTACLE_WIDTH_HOUSE as f32; //Horizontal space
        }

        manager
    }

    pub fn update(&mut self) -> Obstacle {
        // Move osbtacles closer to Santa
        self.obstacles
            .iter_mut()
            .for_each(|osbtacle| osbtacle.update());

        //TODO: Fix this shiz
        // Remove obstacles beyond the screen
        self.obstacles
            .retain(|osbtacle| osbtacle.position.x >= -1.0 * OBSTACLE_WIDTH_HOUSE as f32); //this width can be stored on the obstacle

        // Add new obstacles ( to fill in for the ones removed)
        let num_of_new_obstacles = 
            STARTING_NUMBER_OF_OBSTACLES - (self.obstacles.len() as f64 / 2.0) as usize;
        let x_pos =
            self.obstacles.last().unwrap().position.x + MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES;


        let resources = RESOURCES.get().unwrap();
        // let texture = &resources.house_texture;

        for _ in 0..num_of_new_obstacles {
            self.add_obstacle(x_pos, resources.house_texture.clone());
            self.number_of_cleared += 1;
        }

        self.get_nearest_obstacle() // to do ...
    }

    pub fn draw(&self) {
        let score = format!("Score: {}", self.number_of_cleared);
        self.obstacles.iter().for_each(|obstacle| obstacle.draw());
        draw_text(
            score.as_str(),
            screen_width() / 2.0 - 90.0,
            200.0,
            40.0,
            WHITE,
        );
    }

    fn get_nearest_obstacle(&self) -> Obstacle {
        // Return Nearest?
        self.obstacles[0].clone()
    }

    fn add_obstacle(&mut self, x_pos: f32, texture: Texture2D) {
        self.obstacles.push(Obstacle::new(
            vec2(x_pos, screen_height() / 4.0),
            None,
            texture,
        ));
    }
}