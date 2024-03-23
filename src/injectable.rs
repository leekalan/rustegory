/*!
the semantic opposite of a wrapper, it is automatically implemented and is just the object and generic swapped

it is recomended to implement wrapper as injectable will then be automatically implemented

the 4 trait variants are as follows:

* [`Injectable`]
* [`InjectableRef`]
* [`TryInjectable`]
* [`TryInjectableRef`]

*/

use crate::wrapper::{TryWrapper, TryWrapperRef, Wrapper, WrapperRef};

pub trait Injectable<T, R> {
    fn inject(self, wrapper: T) -> R;
}

pub trait InjectableRef<'a, T, R: 'a> {
    fn inject_ref(&'a self, wrapper: T) -> R;
}

pub trait TryInjectable<T, R> {
    type Error;
    fn try_inject(self, wrapper: T) -> Result<R, Self::Error>;
}

pub trait TryInjectableRef<'a, T, R: 'a> {
    type Error;
    fn try_inject_ref(&'a self, wrapper: T) -> Result<R, Self::Error>;
}

impl<T: Wrapper<U, R>, U, R> Injectable<T, R> for U {
    fn inject(self, wrapper: T) -> R {
        wrapper.wrap(self)
    }
}

impl<'a, T: WrapperRef<'a, U, R>, U, R: 'a> InjectableRef<'a, T, R> for U {
    fn inject_ref(&'a self, wrapper: T) -> R {
        wrapper.wrap_ref(self)
    }
}

impl<T: TryWrapper<U, R>, U, R> TryInjectable<T, R> for U {
    type Error = T::Error;
    fn try_inject(self, wrapper: T) -> Result<R, Self::Error> {
        wrapper.try_wrap(self)
    }
}

impl<'a, T: TryWrapperRef<'a, U, R>, U, R: 'a> TryInjectableRef<'a, T, R> for U {
    type Error = T::Error;
    fn try_inject_ref(&'a self, wrapper: T) -> Result<R, Self::Error> {
        wrapper.try_wrap_ref(self)
    }
}
