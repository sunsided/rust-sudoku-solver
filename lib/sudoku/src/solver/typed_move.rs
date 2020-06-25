use crate::game::Move;

pub struct TypedMove {
    pub r#move: Move,
    pub trivial: bool
}

impl TypedMove {
    pub fn new(r#move: Move, trivial: bool) -> TypedMove {
        TypedMove { r#move, trivial }
    }
}
