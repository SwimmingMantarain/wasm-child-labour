
use std::default;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

use crate::context::{ContextType, ContextWindow};
use crate::utilities::*;

#[derive(Clone)]
struct GameButton {
    text: &'static str,
    pos: Vec2,
    action: FuncTyp,
    context_action: Option<ContextType>
}

impl GameButton {
    fn default() -> GameButton {
        GameButton { text: "Back", pos: vec2(0., 0.), action: FuncTyp::RevContext(revert_context), context_action: None}
    }

    fn new(text: &'static str, pos: Vec2, action: FuncTyp, context_action: Option<ContextType>) -> GameButton {
        GameButton { text: text, pos: pos, action: action, context_action: context_action}
    }
}

#[derive(Clone)]
struct Menu {
    menu_buttons: Vec<GameButton>,
    context: ContextType
}

impl Menu {
    fn default() -> Menu {
        Menu {
            menu_buttons: vec![GameButton::default()],
            context: ContextType::MainMenu
        }
    }

    fn update(&self, context: Option<ContextWindow>, font: Option<&Font>) -> ContextWindow {
        let mut updated_context = context;
        for menu_button in self.menu_buttons.clone() {
            // Alignment black magic
            
            let button_center = get_text_center(menu_button.text, font, 45, 1., 0.);
            let button_adjusted_pos = vec2(
                screen_width() / 2. - button_center.x,
                screen_height() / 2. - button_center.y + menu_button.pos.y
            );

            if widgets::Button::new(menu_button.text)/*.size()*/.position(button_adjusted_pos).ui(&mut root_ui()) {
                match menu_button.action {
                    FuncTyp::Simple(func) => { func() },
                    FuncTyp::Context(func) => { updated_context = Some(func(context.expect("Where context window?"), menu_button.context_action.expect("Where context type?"))); },
                    FuncTyp::RevContext(func) => {updated_context = Some(func(context.expect("Where context window?")))}
                    // _ => { println!("How did you get here bro?"); }
                }
            }
        }
        updated_context.expect("Where is updated context window?")
    }
}

pub struct Menus {
    main_menu: Menu,
    settings_menu: Menu,
    credits_menu: Menu,
    pause_menu: Menu,
    gameplay: Menu
}

impl Menus {
    pub fn new() -> Menus {
        Menus {
            main_menu: Menu {
                menu_buttons: vec![
                    GameButton::new("Play", vec2(0., 0.), FuncTyp::Context(change_context), Some(ContextType::GamePlay)),
                    GameButton::new("Settings", vec2(0., 50.), FuncTyp::Context(change_context), Some(ContextType::SettingsMenu)),
                    GameButton::new("Credits", vec2(0., 100.), FuncTyp::Context(change_context), Some(ContextType::CreditsMenu)),
                    GameButton::new("Quit", vec2(0., 150.), FuncTyp::Simple(quit), None)
                ],
                context: ContextType::MainMenu
            },
            settings_menu: Menu {
                menu_buttons: vec![
                    GameButton::default(),
                ],
                context: ContextType::SettingsMenu,
            },
            credits_menu: Menu {
                menu_buttons: vec![
                    GameButton::default()
                ],
                context: ContextType::CreditsMenu,
            },
            pause_menu: Menu {
                menu_buttons: vec![
                    GameButton::default()
                ],
                context: ContextType::PauseMenu,
            },
            gameplay: Menu {
                menu_buttons: vec![
                    GameButton::default()
                ],
                context: ContextType::GamePlay,
            },
        }
    }

    pub fn set_style(&self, font: &Font) {
        let button_style = root_ui()
            .style_builder()
            .with_font(font).unwrap()
            .color(Color::from_rgba(0, 0, 0, 0))
            .color_hovered(Color::from_rgba(0, 0, 0, 0))
            .color_clicked(Color::from_rgba(0, 0, 0, 0))
            .text_color_hovered(Color::from_rgba(255, 0, 0, 255))
            .text_color_clicked(Color::from_rgba(255, 0, 0, 255))
            .text_color(WHITE)
            .font_size(45)
            .build();

        let ui_skin = Skin {
            button_style,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&ui_skin);
    }

    pub fn update(&mut self, context: ContextWindow, font: Option<&Font>) -> ContextWindow {
        let mut updated_context = context;
        if context.curr_context == ContextType::MainMenu {
            updated_context = self.main_menu.update(Some(context), font);
        } else if context.curr_context == ContextType::SettingsMenu {
            updated_context = self.settings_menu.update(Some(context), font);
        } else if context.curr_context == ContextType::CreditsMenu {
            updated_context = self.credits_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::PauseMenu {
            updated_context = self.pause_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::GamePlay {
            updated_context = self.gameplay.update(Some(context), font)
        }

        return updated_context
    }
}