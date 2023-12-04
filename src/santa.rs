use macroquad::prelude::*;
use macroquad::math::Vec2;
use macroquad_platformer::{Actor, World};

use crate::constants::VENUS_GRAVITY;

const SANTA_MAX_SPEED: f32 = 80.0;
pub struct Santa {
    vel: Vec2,
    texture: Texture2D,
    collider: Actor,
}

impl Santa {
    pub async fn new(world: &mut World) -> Santa {
        Santa {
            vel: Vec2::ZERO,
            texture: load_texture("res/santa.png").await.unwrap(),
            collider: world.add_actor(Vec2::ZERO, 30, 100),
        }
    }

    pub fn handle_input(&mut self, world: &mut World) {
        if is_key_down(KeyCode::W) {
            self.vel.y -= 10.0;
        }

        if is_key_down(KeyCode::S) {
            self.vel.y += 10.0;
        }

        self.vel += VENUS_GRAVITY;
        self.vel = self.vel.clamp_length_max(SANTA_MAX_SPEED);

        world.move_h(self.collider, self.vel.x * get_frame_time());
        world.move_v(self.collider, self.vel.y * get_frame_time());
    }

    pub fn draw(&self, world: &World) {
        let pos = world.actor_pos(self.collider);
        draw_texture(&self.texture, pos.x, pos.y, WHITE);
    }
}