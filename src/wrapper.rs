use std::borrow::Borrow;

pub trait Wrapper<U, R> {
    fn wrap(self, generic: U) -> R;
}

pub trait WrapperRef<U: ?Sized, R> {
    fn wrap_ref(self, generic: impl Borrow<U>) -> R;
}