use macroquad::{prelude::*, miniquad::conf::Platform};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Ho ho Ho-ld my bear!"),
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        icon: None,
        platform: Platform::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let santa_texture: Texture2D = load_texture("res/santa.png").await.unwrap();

    loop {
        clear_background(WHITE);

        if is_key_down(KeyCode::Escape) {
            break;
        }

        draw_texture(&santa_texture, 0., 0., WHITE);

        next_frame().await
    }
}