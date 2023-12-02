use constants::*;
use macroquad::prelude::*;
use santa::Santa;

mod santa;
mod constants;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Ho-ho-Hold my bear!"),
        window_width: GAME_SIZE_X as i32,
        window_height: GAME_SIZE_Y as i32,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let game_render_target = render_target(GAME_SIZE_X, GAME_SIZE_Y);

    let mut santa = Santa::new().await;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        let camera = Camera2D {
            // I have no idea why the zoom is this way lmao
            zoom: vec2(1. / GAME_SIZE_X as f32 * 2., 1. / GAME_SIZE_Y as f32 * 2.),
            target: vec2(
                (GAME_SIZE_X as f32 * 0.5f32).floor(),
                (GAME_SIZE_Y as f32 * 0.5f32).floor(),
            ),
            render_target: Some(game_render_target.clone()),
            ..Default::default()
        };

        santa.handle_input();

        set_camera(&camera);

        clear_background(BLACK);
        game_render_target.texture.set_filter(FilterMode::Nearest);

        santa.draw();

        set_default_camera();

        // calculate game view size based on window size
        let game_diff_w = screen_width() / GAME_SIZE_X as f32;
        let game_diff_h = screen_height() / GAME_SIZE_Y as f32;
        let aspect_diff = game_diff_w.min(game_diff_h);

        let scaled_game_size_w = GAME_SIZE_X as f32 * aspect_diff;
        let scaled_game_size_h = GAME_SIZE_Y as f32 * aspect_diff;

        let width_padding = (screen_width() - scaled_game_size_w) * 0.5f32;
        let height_padding = (screen_height() - scaled_game_size_h) * 0.5f32;

        // draw game
        clear_background(BLACK);

        // fit inside window
        draw_texture_ex(
            &game_render_target.texture,
            width_padding,
            height_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_game_size_w, scaled_game_size_h)),
                ..Default::default()
            },
        );


        next_frame().await
    }
}