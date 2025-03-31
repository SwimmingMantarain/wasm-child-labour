use std::str::FromStr;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

use crate::context::{ContextType, ContextWindow};

pub struct Menu {
    menu_buttons: Vec<&'static str>,
    context: ContextType
}

impl Menu {
    fn default() -> Menu {
        Menu {
            menu_buttons: vec!["Template"],
            context: ContextType::MainMenu
        }
    }

    fn update(&self) {
        for menu_button in self.menu_buttons.to_owned() {
            if root_ui().button(None, menu_button) {
                println!("Button Clicked")
            }
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

    pub fn update(&mut self, context: ContextWindow) {
        if context.curr_context == ContextType::MainMenu {
            self.MainMenu.update();
        }
    }
}