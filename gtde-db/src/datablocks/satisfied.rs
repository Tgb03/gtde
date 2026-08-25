pub trait Satisfied {
    type Target;

    fn satisfied_by(&self, data: &Self::Target) -> bool;
}

impl<T: PartialEq> Satisfied for T {
    fn satisfied_by(&self, data: &T) -> bool {
        self == data
    }

    type Target = T;
}
