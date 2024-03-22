use crate::wrapper::{Wrapper, WrapperRef};

pub trait Injectable<T> {
    type Result;

    fn inject(self, wrapper: T) -> Self::Result;
}

pub trait InjectableRef<T> {
    type Result;

    fn inject_ref(&self, wrapper: T) -> Self::Result;
}

impl<T: Wrapper<U>, U> Injectable<T> for U {
    type Result = T::Result;

    fn inject(self, wrapper: T) -> Self::Result {
        wrapper.wrap(self)
    }
}

impl<T: WrapperRef<U>, U> InjectableRef<T> for U {
    type Result = T::Result;

    fn inject_ref(&self, wrapper: T) -> Self::Result {
        wrapper.wrap_ref(self)
    }
}