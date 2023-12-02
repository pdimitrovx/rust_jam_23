use macroquad::{prelude::*, miniquad::conf::Platform};
use santa::Santa;

mod santa;

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
    let mut santa = Santa::new().await;

    loop {
        clear_background(BLACK);

        santa.handle_input();
        santa.draw();

        next_frame().await
    }
}