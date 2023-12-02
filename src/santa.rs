use macroquad::prelude::*;
use macroquad::math::Vec2;

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
            self.vel.y -= 1.0;
        }

        if is_key_down(KeyCode::S) {
            self.vel.y += 1.0;
        }

        self.pos += self.vel * get_frame_time();
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, WHITE);
    }
}