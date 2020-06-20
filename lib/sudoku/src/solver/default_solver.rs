use crate::GameState;

pub struct DefaultSolver {}

impl DefaultSolver {
    pub fn solve(game: &GameState) -> GameState {
        game.fork()
    }
}
