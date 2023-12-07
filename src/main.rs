use constants::*;

use ingame::InGame;
use macroquad::prelude::*;
use resources::init_resources;
use gamestate::{Gamestate, CurrentGameState};

pub mod santa;
mod constants;
mod resources;
pub mod obstacle;
pub mod gamestate;
mod ingame;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Ho-ho-Hold my beer!"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
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