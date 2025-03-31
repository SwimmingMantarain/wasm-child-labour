use std::str::FromStr;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

use crate::context::ContextType;

struct Button {
    // The pos variable is se more like padding than an actual position
    pos: Vec2,
    text: String,
}

pub struct Menu {
    buttons: Vec<Button>,
    context: ContextType
}

impl Menu {
    fn default() -> Menu {
        Menu {
            buttons: vec![Button { pos: vec2(0., 0.), text: String::from_str("Template").expect("String stuff") }],
            context: ContextType::MainMenu
        }
    }
}

pub struct Menus {
    MainMenu: Menu,
    SettingsMenu: Menu,
    CreditsMenu: Menu,
    PauseMenu: Menu,
    Gameplay: Menu
}

impl Menus {
    pub fn new() -> Menus {
        Menus {
            MainMenu: Menu::default(),
            SettingsMenu: Menu::default(),
            CreditsMenu: Menu::default(),
            PauseMenu: Menu::default(),
            Gameplay: Menu::default(),
        }
    }

    pub fn set_style(&self) {
        let button_style = root_ui()
            .style_builder()
            .color(Color::from_rgba(0, 0, 0, 0))
            .color_clicked(Color::from_rgba(255, 255, 255, 100))
            .text_color(WHITE)
            .font_size(40)
            .build();

        let label_style = root_ui()
            .style_builder()
            .text_color(WHITE)
            .font_size(32)
            .build();

        let ui_skin = Skin {
            button_style,
            label_style,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&ui_skin);
    }

    pub fn update(&mut self) {
        // TODO: Properly implement button positioning and rendering
    }
}