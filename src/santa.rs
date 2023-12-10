use macroquad::{prelude::*, experimental::animation::Animation};
use macroquad::math::Vec2;
use macroquad::experimental::animation::AnimatedSprite;

use crate::constants::*;
use crate::resources::RESOURCES;

const SANTA_HEIGHT: u32 = 33;
const SANTA_WIDTH: u32 = 102;
const SANTA_OFFSET: f32 = 5.;

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
            pos: Vec2::new(20., (GAME_SIZE_Y as f32 - SANTA_HEIGHT as f32) / 2.),
            vel: Vec2::ZERO,
            animated_sprite: AnimatedSprite::new(
                SANTA_WIDTH,
                SANTA_HEIGHT,
                &[Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 8,
                    fps: 12,
                }],
                true,
            ),
            rect: Rect::new(SANTA_OFFSET, SANTA_OFFSET, 20.0, 20.0),
        }
    }

    pub fn check_for_collisions(&self, obstacles: Vec<Rect>) -> bool {
        obstacles.iter().any(|obstacle: &Rect| {
            if obstacle.overlaps(&self.rect) {
                // println!("Overlap found obstacle, rect(santa), pos(santa), type: {:?} // {:?} // {:?} // {:?} //", obstacle, &self.rect, &self.pos, &self.vel);
                true
            } else {
                false
            }
        })
        }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.vel.y -= 10.0;
        }

        if is_key_down(KeyCode::S) {
            self.vel.y += 10.0;
        }

        if is_key_down(KeyCode::A) {
            self.vel.x -= 10.0;
        }

        if is_key_down(KeyCode::D) {
            self.vel.x += 10.0;
        }

        self.vel += VENUS_GRAVITY;
        self.vel = self.vel.clamp_length_max(SANTA_MAX_SPEED);

        self.pos += self.vel * get_frame_time();

        if self.pos.y + SANTA_HEIGHT as f32 > GAME_SIZE_Y as f32 {
            self.pos.y = GAME_SIZE_Y as f32 - SANTA_HEIGHT as f32;
        }

        if self.pos.y < 0. {
            self.pos.y = 0.;
        }

        if self.pos.x + SANTA_WIDTH as f32 > GAME_SIZE_X as f32 {
            self.pos.x = GAME_SIZE_X as f32 - SANTA_WIDTH as f32;
        }

        if self.pos.x < 0. {
            self.pos.x = 0.;
        }

        self.rect.x = self.pos.x + SANTA_OFFSET;
        self.rect.y = self.pos.y + SANTA_OFFSET;
    }

    pub fn draw(&mut self) {
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