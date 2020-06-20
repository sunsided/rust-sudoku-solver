pub trait Visitor<D> {
    type Result;

    fn visit(&self, data: &D) -> Self::Result;
}

pub trait AcceptVisitor<D> {
    fn accept<V: Visitor<D>>(&self, visitor: &V) -> V::Result;
}


#[cfg(test)]
mod tests {
    use crate::{AcceptVisitor, Visitor};

    struct Data {
        pub value: u8
    }

    impl Data {
        pub fn new(value: u8) -> Data { Data { value }}
    }

    struct DoublingVisitor {}

    impl Visitor<Data> for DoublingVisitor {
        type Result = u8;

        fn visit(&self, data: &Data) -> Self::Result {
            data.value * 2
        }
    }

    impl AcceptVisitor<Data> for Data {
        fn accept<V: Visitor<Data>>(&self, visitor: &V) -> <V as Visitor<Data>>::Result {
            visitor.visit(self)
        }
    }

    #[test]
    fn it_works() {
        let data = Data::new(42);
        let visitor = DoublingVisitor { };
        let value = data.accept(&visitor);
        assert_eq!(value, 84);
    }
}
