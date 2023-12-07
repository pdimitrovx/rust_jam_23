#[derive(PartialEq)]
pub enum CurrentGameState {
    InGame,
    Quit,
}

pub trait Gamestate {
    fn init(&mut self);
    fn update(&mut self) -> Option<CurrentGameState>;
    fn draw(&mut self);
}