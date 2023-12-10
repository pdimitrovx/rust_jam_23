use macroquad::math::Vec2;
use macroquad::prelude::*;

use crate::constants::*;

const BUTTON_HEIGHT: u32 = 17;
const BUTTON_WIDTH: u32 = 30;
const BUTTON_X_OFFSET: u32 = 1;
const BUTTON_Y_OFFSET: u32 = 14;

pub struct Button {
    pos: Vec2,
    texture: Texture2D,
    was_pressed: bool,
    was_released: bool,
    dest: Vec2,
}

impl Button {
    pub fn new(pos: Vec2, spritesheet_texture: Texture2D) -> Button {
        Button {
            pos: pos,
            texture: spritesheet_texture,
            was_pressed: false,
            was_released: false,
            dest: Vec2::new(BUTTON_TEXTURE_WIDTH as f32, BUTTON_TEXTURE_HEIGHT as f32),
        }
    }

    pub fn was_pressed(&self) -> bool {
        self.was_released
    }

    fn is_mouse_within_bounds(&self) -> bool {
        let mut current_mouse_position = mouse_position();
        current_mouse_position.0 = current_mouse_position.0 / 2.0;
        current_mouse_position.1 = current_mouse_position.1 / 2.0;

        let is_within_width = current_mouse_position.0 > (self.pos.x + BUTTON_X_OFFSET as f32)
            && current_mouse_position.0
                < (self.pos.x + BUTTON_WIDTH as f32 - BUTTON_X_OFFSET as f32);

        let is_within_height = current_mouse_position.1 > self.pos.y + BUTTON_Y_OFFSET as f32
            && current_mouse_position.1 < self.pos.y + BUTTON_TEXTURE_HEIGHT as f32;

        is_within_width && is_within_height
    }

    pub fn update(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            if self.is_mouse_within_bounds() {
                self.was_released = true;
            }
        } else {
            self.was_released = false;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if self.is_mouse_within_bounds() {
                self.was_pressed = true;
            }
        } else {
            self.was_pressed = false;
        }
    }

    pub fn draw(&mut self) {
        let source_rect = if self.was_pressed {
            Rect::new(
                BUTTON_TEXTURE_WIDTH as f32,
                0.0,
                BUTTON_TEXTURE_WIDTH as f32,
                BUTTON_TEXTURE_HEIGHT as f32,
            )
        } else {
            Rect::new(
                0.0,
                0.0,
                BUTTON_TEXTURE_WIDTH as f32,
                BUTTON_TEXTURE_HEIGHT as f32,
            )
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
            },
        );
    }
}
