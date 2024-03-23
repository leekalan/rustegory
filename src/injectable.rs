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

pub trait InjectableRef<T, R> {
    fn inject_ref(&self, wrapper: T) -> R;
}

pub trait TryInjectable<T, R> {
    type Error;
    fn try_inject(self, wrapper: T) -> Result<R, Self::Error>;
}

pub trait TryInjectableRef<T, R> {
    type Error;
    fn try_inject_ref(&self, wrapper: T) -> Result<R, Self::Error>;
}

impl<T: Wrapper<U, R>, U, R> Injectable<T, R> for U {
    fn inject(self, wrapper: T) -> R {
        wrapper.wrap(self)
    }
}

impl<T: WrapperRef<U, R>, U, R> InjectableRef<T, R> for U {
    fn inject_ref(&self, wrapper: T) -> R {
        wrapper.wrap_ref(self)
    }
}

impl<T: TryWrapper<U, R>, U, R> TryInjectable<T, R> for U {
    type Error = T::Error;
    fn try_inject(self, wrapper: T) -> Result<R, Self::Error> {
        wrapper.try_wrap(self)
    }
}

impl<T: TryWrapperRef<U, R>, U, R> TryInjectableRef<T, R> for U {
    type Error = T::Error;
    fn try_inject_ref(&self, wrapper: T) -> Result<R, Self::Error> {
        wrapper.try_wrap_ref(self)
    }
}
