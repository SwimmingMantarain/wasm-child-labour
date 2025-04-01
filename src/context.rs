// Helper struct for managing window locations (e.g., settings menu, credits, play, pause, etc.)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContextType {
    MainMenu,
    SettingsMenu,
    CreditsMenu,
    PauseMenu,
    GamePlay
}

#[derive(Clone, Copy)]
pub struct ContextWindow {
    pub curr_context: ContextType,
    pub prev_context: ContextType, // Only to use if curr_context used to be GamePlay
}

impl ContextWindow {
    pub fn new() -> ContextWindow {
        ContextWindow {
            curr_context: ContextType::MainMenu,
            prev_context: ContextType::MainMenu
        }
    }

    pub fn update_context(&mut self, new_context: ContextType) {
        self.prev_context = self.curr_context;
        self.curr_context = new_context;
    }
}