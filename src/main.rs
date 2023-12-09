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
pub mod background;

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
