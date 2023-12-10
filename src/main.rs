use constants::*;

use gamestate::{CurrentGameState, Gamestate};
use ingame::InGame;
use macroquad::prelude::*;
use main_menu::MainMenu;
use resources::init_resources;

pub mod background;
pub mod button;
mod constants;
pub mod gamestate;
mod ingame;
mod main_menu;
pub mod obstacle;
mod resources;
pub mod santa;
mod sound_engine;

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

    let mut sound = sound_engine::SoundEngine::new().await;

    let mut ingame = InGame::new();
    let mut main_menu = MainMenu::new();

    let mut current_gamestate: &mut dyn Gamestate = &mut main_menu;
    sound.play(sound_engine::Cues::MusicMenu);

    loop {
        if let Some(next_gamestate) = current_gamestate.update(&mut sound) {
            match next_gamestate {
                CurrentGameState::Quit => break,
                CurrentGameState::InGame => current_gamestate = &mut ingame,
            }

            current_gamestate.init(&mut sound);
            continue;
        }

        current_gamestate.draw();

        next_frame().await
    }
}
