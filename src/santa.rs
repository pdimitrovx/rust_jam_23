use macroquad::{prelude::*, experimental::animation::Animation};
use macroquad::math::Vec2;
use macroquad::experimental::animation::AnimatedSprite;

use crate::constants::{VENUS_GRAVITY, GAME_SIZE_Y};
use crate::resources::RESOURCES;

const SANTA_MAX_SPEED: f32 = 80.0;
pub struct Santa {
    pos: Vec2,
    vel: Vec2,
    animated_sprite: AnimatedSprite,
    rect: Rect,
}

impl Santa {
    pub fn new() -> Santa {
        Santa {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            animated_sprite: AnimatedSprite::new(
                102,
                33,
                &[Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 8,
                    fps: 12,
                }],
                true,
            ),
            rect: Rect::new(2., 2., 102., 33.),
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.vel.y -= 10.0;
        }

        if is_key_down(KeyCode::S) {
            self.vel.y += 10.0;
        }

        self.vel += VENUS_GRAVITY;
        self.vel = self.vel.clamp_length_max(SANTA_MAX_SPEED);

        self.pos += self.vel * get_frame_time();

        if self.pos.y + self.rect.h > GAME_SIZE_Y as f32 {
            self.pos.y = GAME_SIZE_Y as f32 - self.rect.h;
        }

        if self.pos.y < 0. {
            self.pos.y = 0.;
        }
    }

    pub fn draw(&mut self) {
        println!("pos: {:?}", self.pos);

        draw_texture_ex(
            &RESOURCES.get().unwrap().santa_texture,
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
}