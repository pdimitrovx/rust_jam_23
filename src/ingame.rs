use crate::background::GameBackground;
use crate::button::Button;
use crate::gamestate::{CurrentGameState, Gamestate};
use crate::obstacle::{ObstacleManager, MAX_OBSTACLE_SPEED, MIN_OBSTACLE_SPEED};
use crate::resources::RESOURCES;
use crate::santa::Santa;
use crate::sound_engine::{Cues, SoundEngine};
use crate::{constants::*, sound_engine};

use macroquad::prelude::*;
// use macroquad::rand::gen_range;

pub struct InGame {
    obstacle_manager: ObstacleManager,
    background: GameBackground,
    santa: Santa,
    game_over: bool,
    quit_button: Button,
    current_speed: f32,
}

impl InGame {
    pub fn new() -> InGame {
        let button_pos_x = (WINDOW_WIDTH as f32 / 2.0) - (BUTTON_TEXTURE_WIDTH as f32 / 2.0);
        InGame {
            obstacle_manager: ObstacleManager::new(),
            background: GameBackground::new(MIN_OBSTACLE_SPEED),
            santa: Santa::new(),
            game_over: false,
            quit_button: Button::new(
                Vec2::new(button_pos_x, 300.0),
                RESOURCES
                    .get()
                    .unwrap()
                    .quit_button_spritesheet_texture
                    .clone(),
            ),
            current_speed: MIN_OBSTACLE_SPEED,
        }
    }
}

impl InGame {
    fn draw_score(&self) {
        if !self.game_over {
            let score = format!("Score: {}", self.obstacle_manager.get_num_houses_cleared());
            draw_text(score.as_str(), 5.0, 15.0, 25.0, WHITE);
        } else {
            let score = format!("Score: {}", self.obstacle_manager.get_num_houses_cleared());
            draw_text(score.as_str(), 420.0, 200.0, 35.0, RED);
        }
    }

    fn draw_game_over_ui(&mut self) {
        draw_text("Game Over!", 380.0, 150.0, 50.0, RED);
        self.quit_button.draw();
    }
}

impl Gamestate for InGame {
    fn init(&mut self, sound: &mut SoundEngine) {
        sound.play(sound_engine::Cues::MusicGame);
        sound.stop(sound_engine::Cues::MusicMenu);
        self.obstacle_manager = ObstacleManager::new();
        self.santa = Santa::new();
    }

    fn update(&mut self, sound: &mut SoundEngine) -> Option<CurrentGameState> {
        if is_key_down(KeyCode::Escape) {
            return Some(CurrentGameState::Quit);
        }

        if !self.game_over {
            self.obstacle_manager.update();
            self.santa.update();
            self.background.update();

            self.current_speed = MIN_OBSTACLE_SPEED
                + (self.obstacle_manager.get_num_houses_cleared() as f32 / 100.0);

            if self.current_speed > MAX_OBSTACLE_SPEED as f32 {
                self.current_speed = MAX_OBSTACLE_SPEED as f32;
            }

            self.obstacle_manager.update_speed(self.current_speed);
            self.background.update_speed(self.current_speed);

            sound.trigger_ho(self.obstacle_manager.get_num_houses_cleared());

            if self.obstacle_manager.has_air_obstacle() {
                sound.play(Cues::SfxUfo);
            } else {
                sound.stop(Cues::SfxUfo);
            }

            if self
                .santa
                .check_for_collisions(self.obstacle_manager.get_obstacle_rects())
            {
                sound.play(Cues::SfxCrash);
                self.game_over = true;
            }
        } else {
            self.quit_button.update();

            if self.quit_button.was_pressed() {
                sound.play(Cues::SfxClick);
                return Some(CurrentGameState::Quit);
            }
        }

        None
    }

    fn draw(&mut self) {
        let mut camera = Camera2D::from_display_rect(Rect::new(
            0.0,
            0.0,
            GAME_SIZE_X as f32,
            GAME_SIZE_Y as f32,
        ));

        camera.render_target = Some(render_target(GAME_SIZE_X, GAME_SIZE_Y));

        set_camera(&camera);

        // DRAW!
        self.background.draw();
        self.obstacle_manager.draw();
        self.santa.draw();

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

        if self.game_over {
            self.draw_game_over_ui();
        }

        self.draw_score();
    }
}
