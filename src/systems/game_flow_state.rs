#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Starting,
    Running,
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Paused
    }
}
