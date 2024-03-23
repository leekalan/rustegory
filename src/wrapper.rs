/*!
a cheap wrapper that construct a new type, generic over the input type

the 4 trait variants are as follows:

* a [`Wrapper`] is a cheap wrapper that constructs a new type, generic over the input type
* a [`WrapperRef`] is a cheap wrapper that constructs a new type that holds a reference to the input, generic over the input type
* a [`TryWrapper`] is a cheap wrapper that constructs a new type, generic over the input type, that can fail
* a [`TryWrapperRef`] is a cheap wrapper that constructs a new type that holds a reference to the input, generic over the input type, that can fail

*/

use std::borrow::Borrow;

pub trait Wrapper<U, R> {
    fn wrap(self, generic: U) -> R;
}

pub trait WrapperRef<'a, U: ?Sized, R: 'a> {
    fn wrap_ref(self, generic: &'a impl Borrow<U>) -> R;
}

pub trait TryWrapper<U, R> {
    type Error;
    fn try_wrap(self, generic: U) -> Result<R, Self::Error>;
}

pub trait TryWrapperRef<'a, U: ?Sized, R: 'a> {
    type Error;
    fn try_wrap_ref(self, generic: &'a impl Borrow<U>) -> Result<R, Self::Error>;
}
