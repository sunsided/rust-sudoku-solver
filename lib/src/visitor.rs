pub trait Visitor<D> {
    type Result;

    fn visit(&self, data: &D) -> Self::Result;
}

pub trait AcceptVisitor<D> {
    fn accept<V: Visitor<D>>(&self, visitor: &V) -> V::Result;
}
