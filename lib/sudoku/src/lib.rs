extern crate core;

#[macro_use]
mod macros;
mod game;
pub mod solver;
pub mod visualization;

pub use game::Game;
pub use game::GameState;
pub use game::IndexBitSet;
pub use game::Placement;
pub use game::State;
pub use game::ValueBitSet;

pub mod prelude {
    pub use crate::game::prelude::*;
}
