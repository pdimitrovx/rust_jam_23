use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{
    constants::{BACKGROUND_WIDTH, GAME_SIZE_X, GAME_SIZE_Y},
    resources::RESOURCES,
};

pub struct GameBackground {
    houses_position1: f32,
    houses_position2: f32,
    trees_position1: f32,
    trees_position2: f32,
    speed: f32,
}

impl GameBackground {
    pub fn new(speed: f32) -> Self {
        Self {
            houses_position1: 0.0,
            houses_position2: BACKGROUND_WIDTH as f32,
            trees_position1: 0.0,
            trees_position2: BACKGROUND_WIDTH as f32,
            speed,
        }
    }

    pub fn update_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn update(&mut self) {
        self.houses_position1 -= self.speed / 2.0;
        self.houses_position2 -= self.speed / 2.0;
        self.trees_position1 -= self.speed;
        self.trees_position2 -= self.speed;

        if self.houses_position1 <= -3.75 * GAME_SIZE_X as f32 {
            self.houses_position1 = GAME_SIZE_X as f32 - 0.0;
        }
        if self.houses_position2 <= -3.75 * GAME_SIZE_X as f32 {
            self.houses_position2 = GAME_SIZE_X as f32 - 0.0;
        }
        if self.trees_position1 <= -3.75 * GAME_SIZE_X as f32 {
            self.trees_position1 = GAME_SIZE_X as f32 - 0.0;
        }
        if self.trees_position2 <= -3.75 * GAME_SIZE_X as f32 {
            self.trees_position2 = GAME_SIZE_X as f32 - 0.0;
        }
    }

    pub fn draw(&self) {
        // Houses part 1
        draw_texture_ex(
            &RESOURCES.get().unwrap().background_houses_texture,
            self.houses_position1,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BACKGROUND_WIDTH as f32, GAME_SIZE_Y as f32)),
                flip_y: false,
                ..Default::default()
            },
        );

        // Houses part 2
        draw_texture_ex(
            &RESOURCES.get().unwrap().background_houses_texture,
            self.houses_position2,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BACKGROUND_WIDTH as f32, GAME_SIZE_Y as f32)),
                flip_y: false,
                ..Default::default()
            },
        );

        // Trees layer part 1
        draw_texture_ex(
            &RESOURCES.get().unwrap().background_trees_texture,
            self.trees_position1,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BACKGROUND_WIDTH as f32, GAME_SIZE_Y as f32)),
                flip_y: false,
                ..Default::default()
            },
        );

        // Trees layer part 2
        draw_texture_ex(
            &RESOURCES.get().unwrap().background_trees_texture,
            self.trees_position2,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BACKGROUND_WIDTH as f32, GAME_SIZE_Y as f32)),
                flip_y: false,
                ..Default::default()
            },
        );
    }
}
