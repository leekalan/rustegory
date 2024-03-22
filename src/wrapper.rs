use std::borrow::Borrow;

pub trait Wrapper<U> {
    type Result;

    fn wrap(self, generic: U) -> Self::Result;
}

pub trait WrapperRef<U: ?Sized> {
    type Result;

    fn wrap_ref(self, generic: impl Borrow<U>) -> Self::Result;
}