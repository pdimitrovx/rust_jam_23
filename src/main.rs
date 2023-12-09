use constants::*;

use gamestate::{CurrentGameState, Gamestate};
use ingame::InGame;
use macroquad::prelude::*;
use resources::{init_resources, RESOURCES};

mod constants;
pub mod gamestate;
mod ingame;
pub mod obstacle;
mod resources;
pub mod santa;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Ho-ho-Hold my beer!"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

pub struct GameBackground {
    position1: f32,
    position2: f32,
}

impl GameBackground {
    pub fn new() -> Self {
        Self {
            position1: 0.0,
            position2: BACKGROUND_WIDTH as f32,
        }
    }

    pub fn update(&mut self) {
        self.position1 -= BACKGROUND_SPEED;
        self.position2 -= BACKGROUND_SPEED;

        if self.position1 <= -3.75 * GAME_SIZE_X as f32 {
            self.position1 = GAME_SIZE_X as f32 - 0.0;
        }
        if self.position2 <= -3.75 * GAME_SIZE_X as f32 {
            self.position2 = GAME_SIZE_X as f32 - 0.0;
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &RESOURCES.get().unwrap().background_texture,
            self.position1,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BACKGROUND_WIDTH as f32, GAME_SIZE_Y as f32)),
                flip_y: false,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &RESOURCES.get().unwrap().background_texture,
            self.position2,
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

#[macroquad::main(window_conf)]
async fn main() {
    init_resources().await;

    let ingame = InGame::new();
    let mut current_gamestate = ingame;

    loop {
        if let Some(next_gamestate) = current_gamestate.update() {
            if next_gamestate == CurrentGameState::Quit {
                break;
            }
        }

        current_gamestate.draw();

        next_frame().await
    }
}
