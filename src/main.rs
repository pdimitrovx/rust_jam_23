use constants::*;

use gamestate::{CurrentGameState, Gamestate};
use ingame::InGame;
use macroquad::prelude::*;
use main_menu::MainMenu;
use resources::{init_resources, RESOURCES};

mod constants;
pub mod gamestate;
mod ingame;
mod main_menu;
pub mod obstacle;
mod resources;
pub mod santa;
pub mod background;
pub mod button;

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

    let mut ingame = InGame::new();
    let mut main_menu = MainMenu::new();

    let mut current_gamestate: &mut dyn Gamestate = &mut main_menu;

    loop {
        if let Some(next_gamestate) = current_gamestate.update() {
            match next_gamestate {
                CurrentGameState::Quit => break,
                CurrentGameState::InGame => current_gamestate = &mut ingame,
            }

            current_gamestate.init();
            continue;
        }

        current_gamestate.draw();

        next_frame().await
    }
}
