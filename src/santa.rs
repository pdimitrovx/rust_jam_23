use macroquad::{prelude::*, experimental::animation::Animation};
use macroquad::math::Vec2;
use macroquad_platformer::{Actor, World};
use macroquad::experimental::animation::AnimatedSprite;

use crate::constants::VENUS_GRAVITY;

const SANTA_MAX_SPEED: f32 = 80.0;
pub struct Santa {
    vel: Vec2,
    animated_sprite: AnimatedSprite,
    texture: Texture2D,
    collider: Actor,
}

impl Santa {
    pub async fn new(world: &mut World) -> Santa {
        Santa {
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

    pub fn draw(&mut self, world: &World) {
        let pos = world.actor_pos(self.collider);
        draw_texture_ex(
            &self.texture,
            pos.x, 
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                ..Default::default()
            }
        );
        self.animated_sprite.update();

        // draw_texture(&self.animated_sprite.frame(), pos.x, pos.y, WHITE);
    }
}