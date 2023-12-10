use crate::sound_engine::SoundEngine;

#[derive(PartialEq)]
pub enum CurrentGameState {
    InGame,
    Quit,
}

pub trait Gamestate {
    fn init(&mut self);
    fn update(&mut self, sound: &SoundEngine) -> Option<CurrentGameState>;
    fn draw(&mut self);
}
