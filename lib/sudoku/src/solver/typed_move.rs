use crate::game::Move;

pub struct TypedMove {
    pub r#move: Move,
    pub branch: bool
}

impl TypedMove {
    pub fn new(r#move: Move, branch: bool) -> TypedMove {
        TypedMove { r#move, branch }
    }
}
