use crate::context::{ContextType, ContextWindow};

// Helper function for quitting the game
pub fn quit() {
    std::process::exit(1)
}

// Helper enum for using function in GameButton structs
#[derive(Clone, Copy)]
pub enum FuncTyp {
    Simple(fn ()),
    Context(fn (ContextWindow, ContextType) -> ContextWindow),
    RevContext(fn (ContextWindow) -> ContextWindow)
}

// Empty function for black magic purposes
pub fn empty() {}

// Helper function for changing context
pub fn change_context(context: ContextWindow, new_context: ContextType) -> ContextWindow {
    let mut new_ctx_win = context;

    new_ctx_win.update_context(new_context);

    new_ctx_win
}

// Helper function for more black magic purposes
pub fn revert_context(context: ContextWindow) -> ContextWindow {
    let mut new_ctx_win = context;

    new_ctx_win.revert_context();

    new_ctx_win
}