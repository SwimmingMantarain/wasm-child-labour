
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

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
        GameButton { text: "Template", pos: vec2(0., 0.), action: FuncTyp::Simple(empty), context_action: None}
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

    fn update(&self, context: Option<ContextWindow>) -> ContextWindow {
        let mut updated_context = context;
        for menu_button in self.menu_buttons.clone() {
            if root_ui().button(None, menu_button.text) {
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
                    GameButton::new("Back", vec2(0., 0.), FuncTyp::RevContext(revert_context), None)
                ],
                context: ContextType::SettingsMenu
            },
            credits_menu: Menu {
                menu_buttons: vec![
                    GameButton::new("Back", vec2(0., 0.), FuncTyp::RevContext(revert_context), None)
                ],
                context: ContextType::CreditsMenu,
            },
            pause_menu: Menu {
                menu_buttons: vec![
                    GameButton::new("Back", vec2(0., 0.), FuncTyp::RevContext(revert_context), None)
                ],
                context: ContextType::PauseMenu,
            },
            gameplay: Menu {
                menu_buttons: vec![
                    GameButton::new("Back", vec2(0., 0.), FuncTyp::RevContext(revert_context), None)
                ],
                context: ContextType::GamePlay,
            },
        }
    }

    pub fn set_style(&self) {
        let button_style = root_ui()
            .style_builder()
            .color(Color::from_rgba(0, 0, 0, 0))
            .color_clicked(Color::from_rgba(0, 255, 0, 128))
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

    pub fn update(&mut self, context: ContextWindow) -> ContextWindow {
        let mut updated_context = context;
        if context.curr_context == ContextType::MainMenu {
            updated_context = self.main_menu.update(Some(context));
        } else if context.curr_context == ContextType::SettingsMenu {
            updated_context = self.settings_menu.update(Some(context));
        } else if context.curr_context == ContextType::CreditsMenu {
            updated_context = self.credits_menu.update(Some(context))
        } else if context.curr_context == ContextType::PauseMenu {
            updated_context = self.pause_menu.update(Some(context))
        } else if context.curr_context == ContextType::GamePlay {
            updated_context = self.gameplay.update(Some(context))
        }

        return updated_context
    }
}