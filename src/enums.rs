#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartNewTurnState,
    PlayerTurnState,
    NPCsTurnState
}
