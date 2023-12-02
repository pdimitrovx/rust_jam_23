use macroquad::prelude::*;
use macroquad::math::Vec2;

use crate::constants::VENUS_GRAVITY;

const SANTA_MAX_SPEED: f32 = 80.0;


pub struct Santa {
    pos: Vec2,
    vel: Vec2,
    texture: Texture2D,
}

impl Santa {
    pub async fn new() -> Santa {
        Santa {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            texture: load_texture("res/santa.png").await.unwrap(),
        }
    }

    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::W) {
            self.vel.y -= 10.0;
        }

        if is_key_down(KeyCode::S) {
            self.vel.y += 10.0;
        }

        self.vel += VENUS_GRAVITY;
        self.vel = self.vel.clamp_length_max(SANTA_MAX_SPEED);
        self.pos += self.vel * get_frame_time();
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, WHITE);
    }
}