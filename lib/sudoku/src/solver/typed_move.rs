use crate::game::Move;

#[derive(Debug)]
pub struct TypedMove {
    pub r#move: Move,
    pub is_branching: bool
}

impl TypedMove {
    pub fn new(r#move: Move, is_branching: bool) -> TypedMove {
        TypedMove { r#move, is_branching }
    }
}

impl Clone for TypedMove {
    fn clone(&self) -> Self {
        TypedMove { r#move: self.r#move.clone(), is_branching: self.is_branching }
    }
}