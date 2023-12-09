use macroquad::{prelude::*, experimental::animation::Animation};
use macroquad::math::Vec2;
use macroquad::experimental::animation::AnimatedSprite;

use crate::constants::*;
use crate::resources::RESOURCES;

const BUTTON_HEIGHT: u32 = 17;
const BUTTON_WIDTH: u32 = 30;
const BUTTON_X_OFFSET: u32 = 1;
const BUTTON_Y_OFFSET: u32 = 14;

pub struct Button {
    pos: Vec2,
    texture: Texture2D,
    was_pressed: bool,
    dest: Vec2,
}

impl Button {
    pub fn new(pos: Vec2, spritesheet_texture: Texture2D) -> Button {
        Button {
            pos: pos,
            texture: spritesheet_texture,
            was_pressed: false,
            dest: Vec2::new(BUTTON_TEXTURE_WIDTH as f32, BUTTON_TEXTURE_HEIGHT as f32),
        }
    }

    pub fn was_pressed(&self) -> bool {
        self.was_pressed
    }

    pub fn update(&mut self) {

        if is_mouse_button_released(MouseButton::Left) {

            let mut current_mouse_position = mouse_position();
            current_mouse_position.0 = current_mouse_position.0 / 2.0;
            current_mouse_position.1 = current_mouse_position.1 / 2.0;

            let is_within_width = current_mouse_position.0 > (self.pos.x + BUTTON_X_OFFSET as f32) &&
                current_mouse_position.0 < (self.pos.x + BUTTON_WIDTH as f32 - BUTTON_X_OFFSET as f32);

            let is_within_height = current_mouse_position.1 > self.pos.y + BUTTON_Y_OFFSET as f32 &&
                current_mouse_position.1 < self.pos.y + BUTTON_HEIGHT as f32;

            if is_within_width && is_within_height {
                self.was_pressed = true;
            }
        } else {
            self.was_pressed = false;
        }
    }

    pub fn draw(&mut self) {

        let source_rect = if self.was_pressed {
            Rect::new(BUTTON_TEXTURE_WIDTH as f32, BUTTON_TEXTURE_HEIGHT as f32, BUTTON_TEXTURE_WIDTH as f32, BUTTON_TEXTURE_HEIGHT as f32)
        } else {
            Rect::new(0.0, 0.0, BUTTON_TEXTURE_WIDTH as f32, BUTTON_TEXTURE_HEIGHT as f32)
        };

        draw_texture_ex(
            &self.texture,
            self.pos.x, 
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                dest_size: Some(self.dest),
                ..Default::default()
            }
        );
    }
}