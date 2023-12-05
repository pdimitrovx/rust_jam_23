use constants::*;
use game::Game;
use macroquad::prelude::*;
use macroquad_platformer::World;
use resources::{init_resources, RESOURCES};
use santa::Santa;

mod santa;
mod constants;
mod game;
mod resources;

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

    let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_SIZE_X as f32, GAME_SIZE_Y as f32));

    let mut world = World::new();
    let mut game = Game::new(&mut world);
    let mut santa = Santa::new(&mut world).await;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        santa.handle_input(&mut world);

        camera.render_target = Some(render_target(GAME_SIZE_X, GAME_SIZE_Y));

        set_camera(&camera);
        
        // DRAW!        
        santa.draw(&world);
        let _nearest_obstacle = game.update(&mut world); //TODO can use that to detect colision
        game.draw();

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
        

        next_frame().await
    }
}