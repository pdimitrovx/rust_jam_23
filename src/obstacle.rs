use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::{color::WHITE, math::Vec2, texture::Texture2D};

use crate::constants::GAME_SIZE_X;
use crate::resources::RESOURCES;

pub const MIN_OBSTACLE_SPEED: f32 = 1.5;
pub const MAX_OBSTACLE_SPEED: f32 = 4.0;
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
    pub fn new_ground_obstacle(position: Vec2, speed: Option<f32>, texture: Texture2D) -> Obstacle {
        // pub  fn new(texture_filepath: &str ) -> Obstacle {
        Obstacle {
            pos: position,
            speed: speed.unwrap_or(MIN_OBSTACLE_SPEED),
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

    // pub fn new_air_obstacle(position: Vec2, speed: Option<f32>, texture: Texture2D) -> Obstacle {
    //     // pub  fn new(texture_filepath: &str ) -> Obstacle {
    //     Obstacle {
    //         pos: position,
    //         speed: speed.unwrap_or(MIN_OBSTACLE_SPEED),
    //         animated_sprite: AnimatedSprite::new(
    //             OBSTACLE_WIDTH_HOUSE as u32,
    //             OBSTACLE_HEIGHT_HOUSE as u32,
    //             &[Animation {
    //                 name: "idle".to_string(),
    //                 row: 0,
    //                 frames: 8,
    //                 fps: 12,
    //             }],
    //             true,
    //         ),
    //         texture: texture,
    //         rect: Rect::new(
    //             0.,
    //             0.,
    //             OBSTACLE_WIDTH_HOUSE as f32,
    //             OBSTACLE_HEIGHT_HOUSE as f32,
    //         ),
    //     }
    // }

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
            },
        );
        self.animated_sprite.update();
    }

    fn update(&mut self) {
        self.pos.x -= self.speed;
    }
}

pub struct ObstacleManager {
    ground_obstacles: Vec<Obstacle>,
    air_obstacles: Vec<Obstacle>,
    // increment for each obstacle that goes past, ie score
    number_of_cleared: u32,
}
impl ObstacleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            ground_obstacles: Vec::new(),
            air_obstacles: Vec::new(),
            number_of_cleared: 0,
        };

        let resources = RESOURCES.get().unwrap();

        manager.add_obstacle(
            GAME_SIZE_X as f32,
            Some(resources.get_random_ground_obstacle()),
            None,
        );

        manager
    }

    pub fn update(&mut self) {
        // Move osbtacles closer to Santa
        self.ground_obstacles
            .iter_mut()
            .for_each(|osbtacle| osbtacle.update());

        println!("Obstacles: {}", self.ground_obstacles.len());

        // Remove obstacles beyond the screen
        self.ground_obstacles.retain(|obstacle| {
            let remove = (obstacle.pos.x + OBSTACLE_WIDTH_HOUSE as f32) > 0.0;
            if !remove {
                self.number_of_cleared += 1;
            }
            remove
        });

        // Remove Air obstacles that have passed
        self.air_obstacles.retain(|obstacle| {
            let remove = (obstacle.pos.x + OBSTACLE_WIDTH_HOUSE as f32) > 0.0;
            if !remove {
                self.number_of_cleared += 3; //Triple score increase
            }
            remove
        });

        // Add new obstacles ( to fill in for the ones removed)
        let x_pos = if let Some(last) = self.ground_obstacles.last() {
            last.pos.x + gen_range(MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES, MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES * 400.0)
        } else {
            gen_range(MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES, MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES * 400.0)
        };

        if x_pos < GAME_SIZE_X as f32 {
            println!("Added - x_pos: {}", x_pos);
            let resources = RESOURCES.get().unwrap();
            self.add_obstacle(
                GAME_SIZE_X as f32 + 32.0,
                Some(resources.get_random_ground_obstacle()),
                None,
            );
        }

        println!("Obstacles removed: {}", self.number_of_cleared);
    }

    pub fn draw(&mut self) {
        //let score = format!("Score: {}", self.number_of_cleared);
        self.ground_obstacles
            .iter_mut()
            .for_each(|obstacle| obstacle.draw());
        //draw_text(
        //    score.as_str(),
        //    screen_width() / 2.0 - 90.0,
        //    200.0,
        //    40.0,
        //    WHITE,
        //);
    }

    fn add_obstacle(
        &mut self,
        x_pos: f32,
        ground_texture: Option<Texture2D>,
        air_texture: Option<Texture2D>,
    ) {
        if ground_texture.is_none() && air_texture.is_none() {
            panic!("you are an idion & you called add obstacle without a texture")
        }

        if let Some(ground_texture) = ground_texture {
            self.ground_obstacles.push(Obstacle::new_ground_obstacle(
                vec2(x_pos, screen_height() / 4.0),
                None,
                ground_texture,
            ));
        }

        //TODO: PUT THE AIR TEXTURE IN CORRECT COORDINTAES - maybe use a y range to spawn at slightly random locations
        // if let Some(air_texture) = air_texture{

        //     self.ground_obstacles.push(Obstacle::new(
        //         vec2(x_pos, screen_height() / 4.0),
        //         None,
        //         air_texture,
        //     ));
        // }
    }
}
