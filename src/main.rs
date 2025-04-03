use context::ContextType;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

pub mod menus;
pub mod context;
pub mod utilities;

fn window_conf() -> Conf {
    Conf {
        window_title: "Child Labour: Epilepsy Edition FHD".to_owned(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load stuff
    let menu_font = load_ttf_font("assets/fonts/AmazDooM/AmazDooMLeft2.ttf").await.unwrap();
    //let menu_font = load_ttf_font("assets/fonts/XXII-Scratch/XXII-Scratch.ttf").await.unwrap();
    //let menu_font = load_ttf_font("assets/fonts/Black-Mustang/Black-Mustang.ttf").await.unwrap();

    let mut context = context::ContextWindow::new();
    let mut game_menus = menus::Menus::new();
    game_menus.set_style(&menu_font);

    

    loop {
        clear_background(BLACK);

        context = game_menus.update(context, Some(&menu_font));

        next_frame().await
    }
}
