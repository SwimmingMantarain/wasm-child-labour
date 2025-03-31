use context::ContextType;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

pub mod menus;
pub mod context;

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

    let mut context = context::ContextWindow::new();
    let mut game_menus = menus::Menus::new();
    game_menus.set_style();

    

    loop {
        clear_background(BLACK);

        if context.curr_context == ContextType::MainMenu {
        }
 

        next_frame().await
    }
}