use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Child Labour: Epilepsy Edition FHD".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}