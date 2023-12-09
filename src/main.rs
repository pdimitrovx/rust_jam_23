use constants::*;

use gamestate::{CurrentGameState, Gamestate};
use ingame::InGame;
use macroquad::prelude::*;
use obstacle::BACKGROUND_SPEED;
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
            position2: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.position1 -= BACKGROUND_SPEED;
        self.position2 -= BACKGROUND_SPEED;

        //Ripped of the internet, no clue what it does??
        if self.position1 <= -1.0 * screen_width() {
            self.position1 = screen_width() - 5.0;
        }
        if self.position2 <= -1.0 * screen_width() {
            self.position2 = screen_width() - 5.0;
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &RESOURCES.get().unwrap().background_texture,
            self.position1,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                flip_y: true,
                ..Default::default()
            },
        );

        draw_texture_ex(
            &RESOURCES.get().unwrap().background_texture,
            self.position2,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                flip_y: true,
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
