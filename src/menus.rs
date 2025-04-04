use macroquad::prelude::*;
use macroquad::ui::{Skin, root_ui, widgets};

use crate::context::{ContextType, ContextWindow};
use crate::utilities::*;

#[derive(Clone)]
struct GameButton {
    text: &'static str,
    order: u64,
    action: FuncTyp,
    context_action: Option<ContextType>,
}

impl GameButton {
    fn default() -> GameButton {
        GameButton {
            text: "Back",
            order: 0,
            action: FuncTyp::RevContext(revert_context),
            context_action: None,
        }
    }

    fn new(
        text: &'static str,
        order: u64,
        action: FuncTyp,
        context_action: Option<ContextType>,
    ) -> GameButton {
        GameButton {
            text: text,
            order: order,
            action: action,
            context_action: context_action,
        }
    }
}

#[derive(Clone)]
struct Menu {
    menu_buttons: Vec<GameButton>,
    context: ContextType,
}

impl Menu {
    fn default() -> Menu {
        Menu {
            menu_buttons: vec![GameButton::default()],
            context: ContextType::MainMenu,
        }
    }

    fn update(&self, context: Option<ContextWindow>, font: Option<&Font>) -> ContextWindow {
        let mut updated_context = context;
        for menu_button in self.menu_buttons.clone() {
            // Alignment black magic

            let button_center = get_text_center(menu_button.text, font, 90, 1., 0.);
            let button_dim = measure_text(menu_button.text, font, 90, 1.);
            let button_adjusted_pos = vec2(
                screen_width() / 2. - button_center.x,
                screen_height() / 2. - button_center.y + (button_dim.height * menu_button.order as f32),
            );

            if widgets::Button::new(menu_button.text)
                .position(button_adjusted_pos)
                .ui(&mut root_ui())
                // TODO: add selection with keyboard using:
                // TODO: .selected(true/false)
            {
                match menu_button.action {
                    FuncTyp::Simple(func) => func(),
                    FuncTyp::Context(func) => {
                        updated_context = Some(func(
                            context.expect("Where context window?"),
                            menu_button.context_action.expect("Where context type?"),
                        ));
                    }
                    FuncTyp::RevContext(func) => {
                        updated_context = Some(func(context.expect("Where context window?")))
                    } // _ => { println!("How did you get here bro?"); }
                }
            }
        }
        updated_context.expect("Where is updated context window?")
    }
}

pub struct Menus {
    main_menu: Menu,
    settings_menu: Menu,
    config_graphics_menu: Menu,
    config_audio_menu: Menu,
    config_controls_menu: Menu,
    config_general_menu: Menu,
    credits_menu: Menu,
    pause_menu: Menu,
    gameplay: Menu,
}

impl Menus {
    pub fn new() -> Menus {
        Menus {
            main_menu: Menu {
                menu_buttons: vec![
                    GameButton::new(
                        "Play",
                       0,
                        FuncTyp::Context(change_context),
                        Some(ContextType::GamePlay),
                    ),
                    GameButton::new(
                        "Settings",
                        1,
                        FuncTyp::Context(change_context),
                        Some(ContextType::SettingsMenu),
                    ),
                    GameButton::new(
                        "Credits",
                        2,
                        FuncTyp::Context(change_context),
                        Some(ContextType::CreditsMenu),
                    ),
                    GameButton::new(
                        "Quit",
                        3,
                        FuncTyp::Simple(quit),
                        None
                    ),
                ],
                context: ContextType::MainMenu,
            },
            settings_menu: Menu {
                menu_buttons: vec![
                    GameButton::new(
                        "Graphics",
                        0,
                        FuncTyp::Context(change_context),
                        Some(ContextType::SettingsGraphics)
                    ),
                    GameButton::new(
                        "Sound",
                        1,
                        FuncTyp::Context(change_context),
                        Some(ContextType::SettingsAudio)
                    ),
                    GameButton::new(
                        "Controls",
                        2,
                        FuncTyp::Context(change_context),
                        Some(ContextType::SettingsControls),
                    ),
                    GameButton::new(
                        "General",
                        3,
                        FuncTyp::Context(change_context),
                        Some(ContextType::SettingsGeneral)
                    ),
                    GameButton::new(
                        "Back",
                        4,
                        FuncTyp::RevContext(revert_context),
                        None
                    )
                ],
                context: ContextType::SettingsMenu,
            },
            config_graphics_menu: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::SettingsGraphics,
            },

            config_audio_menu: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::SettingsAudio
            },
            config_controls_menu: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::SettingsControls,
            },

            config_general_menu: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::SettingsGeneral,
            },

            credits_menu: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::CreditsMenu,
            },
            pause_menu: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::PauseMenu,
            },
            gameplay: Menu {
                menu_buttons: vec![GameButton::default()],
                context: ContextType::GamePlay,
            },
        }
    }

    pub fn set_style(&self, font: &Font) {
        let button_style = root_ui()
            .style_builder()
            .with_font(font)
            .unwrap()
            .color(Color::from_rgba(0, 0, 0, 0))
            .color_hovered(Color::from_rgba(0, 0, 0, 0))
            .color_clicked(Color::from_rgba(0, 0, 0, 0))
            .text_color_hovered(Color::from_rgba(255, 0, 0, 255))
            .text_color_clicked(Color::from_rgba(255, 0, 0, 255))
            .text_color(WHITE)
            .font_size(90)
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
        } else if context.curr_context == ContextType::SettingsGraphics {
            updated_context = self.config_graphics_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::SettingsAudio {
            updated_context = self.config_audio_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::SettingsControls {
            updated_context = self.config_controls_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::SettingsGeneral {
            updated_context = self.config_general_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::CreditsMenu {
            updated_context = self.credits_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::PauseMenu {
            updated_context = self.pause_menu.update(Some(context), font)
        } else if context.curr_context == ContextType::GamePlay {
            updated_context = self.gameplay.update(Some(context), font)
        }

        return updated_context;
    }
}
