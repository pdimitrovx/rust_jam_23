use macroquad::experimental::animation::{Animation, AnimatedSprite};
use macroquad::prelude::*;
use macroquad::{color::WHITE, math::Vec2, texture::Texture2D};

use crate::constants::GAME_SIZE_X;
use crate::resources::RESOURCES;

pub const DEFAULT_OBSTACLE_SPEED: f32 = 2.0;
pub const OBSTACLE_WIDTH_HOUSE: i32 = 64;
pub const OBSTACLE_HEIGHT_HOUSE: i32 = 64; //
pub const STARTING_NUMBER_OF_OBSTACLES: usize = 2;
pub const MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES: f32 = 64.0;

#[derive(Clone)]
pub struct Obstacle {
    pub pos: Vec2,
    //example has height, maybe we need height as optional on houses?
    pub speed: f32,
    pub animated_sprite: AnimatedSprite,
    pub texture: Texture2D,
    pub rect: Rect,
}

impl Obstacle {
    pub fn new(position: Vec2, speed: Option<f32>, texture: Texture2D) -> Obstacle {
        // pub  fn new(texture_filepath: &str ) -> Obstacle {
        Obstacle {
            pos: position,
            speed: speed.unwrap_or(DEFAULT_OBSTACLE_SPEED),
            animated_sprite: AnimatedSprite::new(
                OBSTACLE_WIDTH_HOUSE as u32,
                OBSTACLE_HEIGHT_HOUSE as u32,
                &[Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 8,
                    fps: 12,
                }],
                true,
            ),
            texture: texture,
            rect: Rect::new(
                0.,
                0.,
                OBSTACLE_WIDTH_HOUSE as f32,
                OBSTACLE_HEIGHT_HOUSE as f32,
            ),
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.texture,
            self.pos.x, 
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                ..Default::default()
            }
        );
        self.animated_sprite.update();
        }

    fn update(&mut self) {
        self.pos.x -= self.speed;
    }
}

pub struct ObstacleManager {
    obstacles: Vec<Obstacle>,
    // increment for each obstacle that goes past, ie score
    number_of_cleared: u32,
}
impl ObstacleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            obstacles: Vec::new(),
            number_of_cleared: 0,
        };

        //TODO: random (house/ground obst) texture?
        let resources = RESOURCES.get().unwrap();
        // let texture = &resources.house_texture;

        manager.add_obstacle(GAME_SIZE_X as f32, resources.get_random_ground_obstacle());

        manager
    }

    pub fn update(&mut self) {
        // Move osbtacles closer to Santa
        self.obstacles
            .iter_mut()
            .for_each(|osbtacle| osbtacle.update());

        // println!("Obstacles: {}", self.obstacles.len());

        //TODO: Fix this shiz
        // Remove obstacles beyond the screen
        self.obstacles
            .retain(|osbtacle| (osbtacle.pos.x + OBSTACLE_WIDTH_HOUSE as f32) > 0.0); //this width can be stored on the obstacle

        // Add new obstacles ( to fill in for the ones removed)
        if let Some(&ref last) = self.obstacles.last() {
            let x_pos = last.pos.x + MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES;
            
            if x_pos < GAME_SIZE_X as f32 {
                // println!("Added - x_pos: {}", x_pos);
                let resources = RESOURCES.get().unwrap();
                self.add_obstacle(x_pos, resources.get_random_ground_obstacle());
            }
        }
    }

    pub fn draw(&mut self) {
        //let score = format!("Score: {}", self.number_of_cleared);
        self.obstacles.iter_mut().for_each(|obstacle| obstacle.draw());
        //draw_text(
        //    score.as_str(),
        //    screen_width() / 2.0 - 90.0,
        //    200.0,
        //    40.0,
        //    WHITE,
        //);
    }

    fn add_obstacle(&mut self, x_pos: f32, texture: Texture2D) {
        self.obstacles.push(Obstacle::new(
            vec2(x_pos, screen_height() / 4.0),
            None,
            texture,
        ));
    }
}
