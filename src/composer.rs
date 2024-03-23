/*!
a potentially costly operation that takes a generic input type, and composes it within itself

the 4 trait variants are as follows:

* a [`Composer`] is a potentially costly operation that takes a generic input type, and composes it within itself
* a [`ComposerRef`] is a potentially costly operation that takes a reference to a generic input type, and composes it within itself
* a [`TryComposer`] is a potentially costly operation that takes a generic input type, and composes it within itself, that can fail
* a [`TryComposerRef`] is a potentially costly operation that takes a reference to a generic input type, and composes it within itself, that can fail

*/

use std::borrow::Borrow;

pub trait Composer<U> {
    fn compose(&mut self, generic: U) -> &mut Self;
}

pub trait ComposerRef<'a, U: ?Sized>: 'a {
    fn compose_ref(&mut self, generic: &'a impl Borrow<U>) -> &mut Self;
}

pub trait TryComposer<U> {
    type Error;
    fn try_compose(&mut self, generic: U) -> Result<&mut Self, Self::Error>;
}

pub trait TryComposerRef<'a, U: ?Sized>: 'a {
    type Error;
    fn try_compose_ref(&mut self, generic: &'a impl Borrow<U>) -> Result<&mut Self, Self::Error>;
}
