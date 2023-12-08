use crate::{constants::*, GameBackground};
use crate::gamestate::{Gamestate, CurrentGameState};
use crate::obstacle::ObstacleManager;
use crate::santa::Santa;

use macroquad::prelude::*;
// use macroquad::rand::gen_range;

pub struct InGame {
    obstacle_manager: ObstacleManager,
    background: GameBackground,
    santa: Santa,
}

impl InGame {
    pub fn new() -> InGame {
        InGame {
            obstacle_manager: ObstacleManager::new(),
            background: GameBackground::new(),
            santa: Santa::new(), 
        }
    }
}

impl Gamestate for InGame {
    fn init(&mut self) {
        self.obstacle_manager = ObstacleManager::new();
        self.santa = Santa::new();
    }

    fn update(&mut self) -> Option<CurrentGameState> {
        if is_key_down(KeyCode::Escape) {
            return Some(CurrentGameState::Quit);
        }

        self.obstacle_manager.update();
        self.santa.update();
        self.background.update();
        None
    }

    fn draw(&mut self) {

        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_SIZE_X as f32, GAME_SIZE_Y as f32));

        camera.render_target = Some(render_target(GAME_SIZE_X, GAME_SIZE_Y));

        set_camera(&camera);
        
        // DRAW!        
        self.background.draw();
        self.santa.draw();
        self.obstacle_manager.draw();

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

        // draw ui here
    }
}
