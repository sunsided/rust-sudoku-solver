use crate::Board;

pub trait BoardVisitor {
    type Result;

    fn visit(&self, data: &Board) -> Self::Result;
}

pub trait AcceptVisitor {
    fn accept<V: BoardVisitor>(&self, visitor: &V) -> V::Result;
}

impl AcceptVisitor for Board {
    fn accept<V: BoardVisitor>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}
