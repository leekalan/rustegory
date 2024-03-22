use std::borrow::Borrow;

pub trait Composer<U> {
    fn compose(&mut self, generic: U) -> &mut Self;
}

pub trait ComposerRef<U: ?Sized> {
    fn compose_ref(&mut self, generic: impl Borrow<U>) -> &mut Self;
}