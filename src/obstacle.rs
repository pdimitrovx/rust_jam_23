use std::time::{Duration, Instant};

use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::{color::WHITE, math::Vec2, texture::Texture2D};

use crate::constants::{GAME_SIZE_X, GAME_SIZE_Y};
use crate::resources::RESOURCES;

pub const MIN_OBSTACLE_SPEED: f32 = 1.5;
pub const MAX_OBSTACLE_SPEED: f32 = 4.0;
pub const OBSTACLE_WIDTH_GROUND: i32 = 96;
pub const OBSTACLE_HEIGHT_GROUND: i32 = 96; //
pub const OBSTACLE_WIDTH_AIR: i32 = 64;
pub const OBSTACLE_HEIGHT_AIR: i32 = 40; //
pub const GROUND_OBSTACLE_FRAME_COUNT: u32 = 8; //
pub const AIR_OBSTACLE_FRAME_COUNT: u32 = 17; //

pub const STARTING_NUMBER_OF_OBSTACLES: usize = 2;
pub const MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES: f32 = 64.0;

pub const AIR_OBSTACLE_SPAWN_TIMEOUT_SECS: u64 = 10;

#[derive(Clone)]
pub struct Obstacle {
    pub pos: Vec2,
    //example has height, maybe we need height as optional on houses?
    pub speed: f32,
    pub animated_sprite: AnimatedSprite,
    pub texture: Texture2D,
    pub rect: Rect,
    offsets: Vec2,
}

impl Obstacle {
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn new(position: Vec2, speed: Option<f32>, texture: Texture2D, dims: Vec2, offsets: Vec2, frames: u32, tile_width: u32, tile_height: u32, ) -> Obstacle {
        // pub  fn new(texture_filepath: &str ) -> Obstacle {
        Obstacle {
            pos: position,
            speed: speed.unwrap_or(MIN_OBSTACLE_SPEED),
            animated_sprite: AnimatedSprite::new(
                tile_width,
                tile_height,
                &[Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: frames,
                    fps: 12,
                }],
                true,
            ),
            texture: texture,
            rect: Rect::new(
                position.x + offsets.x,
                position.y + offsets.y,
                dims.x,
                dims.y,
            ),
            offsets,
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
        
        self.rect.x = self.pos.x + self.offsets.x;
        self.rect.y = self.pos.y + self.offsets.y;
        // println!("pos: {:?}, rect{:?}", self.pos, self.rect);
    }
}

pub struct ObstacleManager {
    ground_obstacles: Vec<Obstacle>,
    air_obstacles: Vec<Obstacle>,
    // increment for each obstacle that goes past, ie score
    number_of_cleared: u32,
    start_time: Instant,
    current_speed: f32,
}
impl ObstacleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            ground_obstacles: Vec::new(),
            air_obstacles: Vec::new(),
            number_of_cleared: 0,
            start_time: Instant::now(),
            current_speed: MIN_OBSTACLE_SPEED,
        };

        let resources = RESOURCES.get().unwrap();

        manager.add_obstacle(
            GAME_SIZE_X as f32,
            Some(resources.get_random_ground_obstacle()),
            None,
            Vec2::new(30.0, 30.0),
            Vec2::new(17.0, 24.0),
        );

        manager
    }

    pub fn get_obstacle_rects(&self) -> Vec<Rect> {
        let mut rects = self
            .ground_obstacles
            .iter()
            .map(|obstacle| obstacle.rect)
            .collect::<Vec<Rect>>();
        rects.append(
            &mut self
                .air_obstacles
                .iter()
                .map(|obstacle| obstacle.rect)
                .collect::<Vec<Rect>>(),
        );
        rects
    }

    pub fn update_speed(&mut self, speed: f32) {
        self.current_speed = speed;

        for obstacle in self.ground_obstacles.iter_mut() {
            obstacle.set_speed(speed);
        }
    }

    pub fn update(&mut self) {
        // Move osbtacles closer to Santa
        self.ground_obstacles
            .iter_mut()
            .for_each(|osbtacle| osbtacle.update());
        self.air_obstacles
            .iter_mut()
            .for_each(|osbtacle| osbtacle.update());

        // Remove Ground obstacles beyond the screen
        self.ground_obstacles.retain(|obstacle| {
            let remove = (obstacle.pos.x + OBSTACLE_WIDTH_GROUND as f32) > 0.0;
            if !remove {
                self.number_of_cleared += 1;
            }
            remove
        });

        // Remove Air obstacles beyond the screen
        self.air_obstacles.retain(|obstacle| {
            let remove = (obstacle.pos.x + OBSTACLE_WIDTH_AIR as f32) > 0.0;
            if !remove {
                self.number_of_cleared += 3; //Triple score increase
            }
            remove
        });

        // Add new ground obstacles ( to fill in for the ones removed)
        {
            let x_pos_ground_obst = if let Some(last) = self.ground_obstacles.last() {
                last.pos.x
                    + gen_range(
                        MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES,
                        MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES * 400.0,
                    )
            } else {
                gen_range(
                    MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES,
                    MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES * 400.0,
                )
            };

            if x_pos_ground_obst < GAME_SIZE_X as f32 {
                println!("Added - x_pos: {}", x_pos_ground_obst);
                let resources = RESOURCES.get().unwrap();
                self.add_obstacle(
                    GAME_SIZE_X as f32 + 32.0,
                    Some(resources.get_random_ground_obstacle()),
                    None,
                    Vec2::new(30.0, 30.0),
                    Vec2::new(17.0, 24.0),
                );
            }
        }

        // Add new AIR obstacles ( to fill in for the ones removed)
        let elapsed_time = self.start_time.elapsed().as_secs();
        if elapsed_time > AIR_OBSTACLE_SPAWN_TIMEOUT_SECS {
            let x_pos_air_obst = gen_range(
                MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES,
                MIN_HORIZONTAL_SPACE_BETWEEN_OBSTACLES * 4000.0,
            ); // even lower chances of UFO

            if x_pos_air_obst < GAME_SIZE_X as f32 {
                println!(
                    "Added AIR after {} seconds at x_pos: {}",
                    elapsed_time, x_pos_air_obst
                );
                //reset the time
                self.start_time = Instant::now();
                let resources = RESOURCES.get().unwrap();
                self.add_obstacle(
                    GAME_SIZE_X as f32 + 32.0,
                    None,
                    Some(resources.get_random_air_obstacle()),
                    Vec2::new(44.0, 32.0),
                    Vec2::new(10.0, 2.0),
                );
            }
        }
        // println!("Obstacles removed: {}", self.number_of_cleared);
    }

    pub fn get_num_houses_cleared(&self) -> u32 {
        self.number_of_cleared
    }

    pub fn has_air_obstacle(&self) -> bool {
        !self.air_obstacles.is_empty()
    }

    pub fn draw(&mut self) {
        self.ground_obstacles
            .iter_mut()
            .for_each(|obstacle| obstacle.draw());

        self.air_obstacles
            .iter_mut()
            .for_each(|obstacle: &mut Obstacle| obstacle.draw());
    }

    fn add_obstacle(
        &mut self,
        x_pos: f32,
        ground_texture: Option<Texture2D>,
        air_texture: Option<Texture2D>,
        dims: Vec2,
        offsets: Vec2,
    ) {
        if ground_texture.is_none() && air_texture.is_none() {
            panic!("you are an idion & you called add obstacle without a texture")
        }

        if let Some(ground_texture) = ground_texture {
            self.ground_obstacles.push(Obstacle::new(
                vec2(
                    x_pos,
                    GAME_SIZE_Y as f32 - OBSTACLE_HEIGHT_GROUND as f32 * gen_range(1.09, 1.81),
                ), //1.8 max 1.1 min
                Some(self.current_speed),
                ground_texture,
                dims,
                offsets,
                GROUND_OBSTACLE_FRAME_COUNT,
                OBSTACLE_WIDTH_GROUND as u32,
                OBSTACLE_HEIGHT_GROUND as u32,
            ));
        }

        //TODO: PUT THE AIR TEXTURE IN CORRECT COORDINTAES - maybe use a y range to spawn at slightly random locations
        if let Some(air_texture) = air_texture {
            self.air_obstacles.push(Obstacle::new(
                vec2(x_pos, gen_range(0.0, GAME_SIZE_Y as f32 / 2.0)),
                // vec2(x_pos, 0.0),
                Some(MAX_OBSTACLE_SPEED),
                air_texture,
                dims,
                offsets,
                AIR_OBSTACLE_FRAME_COUNT,
                OBSTACLE_WIDTH_AIR as u32,
                OBSTACLE_HEIGHT_AIR as u32,
            ));
        }
    }
}
