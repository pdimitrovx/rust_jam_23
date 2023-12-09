use macroquad::prelude::*;

use crate::constants::*;
use crate::gamestate::{Gamestate, CurrentGameState};
use crate::button::Button;
use crate::resources::RESOURCES;

pub struct MainMenu {
    play_button: Button,
    quit_button: Button,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let button_pos_x = (GAME_SIZE_X as f32 / 2.0) - (BUTTON_TEXTURE_WIDTH as f32 / 2.0);
        let button_offset_y = (GAME_SIZE_Y as f32 / 2.0);
        MainMenu {
            play_button: Button::new(Vec2::new(button_pos_x, button_offset_y), RESOURCES.get().unwrap().play_button_spritesheet_texture.clone()),
            quit_button: Button::new(Vec2::new(button_pos_x, button_offset_y + BUTTON_TEXTURE_HEIGHT as f32), RESOURCES.get().unwrap().quit_button_spritesheet_texture.clone()),
        }
    }
}

impl Gamestate for MainMenu {
    fn init(&mut self) {
    }

    fn update(&mut self) -> Option<CurrentGameState> {
        if is_key_down(KeyCode::Escape) {
            return Some(CurrentGameState::Quit);
        }

        self.play_button.update();
        self.quit_button.update();

        if self.quit_button.was_pressed() {
            return Some(CurrentGameState::Quit)
        }

        None
    }

    fn draw(&mut self) {

        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_SIZE_X as f32, GAME_SIZE_Y as f32));

        camera.render_target = Some(render_target(GAME_SIZE_X, GAME_SIZE_Y));

        set_camera(&camera);
        
        self.play_button.draw();
        self.quit_button.draw();

        set_default_camera();
        clear_background(BLACK);

        let render_target = camera.render_target.unwrap();
        render_target.texture.set_filter(FilterMode::Nearest);

        // fit inside window
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
