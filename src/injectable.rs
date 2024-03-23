use crate::wrapper::{Wrapper, WrapperRef};

pub trait Injectable<T, R> {
    fn inject(self, wrapper: T) -> R;
}

pub trait InjectableRef<T, R> {
    fn inject_ref(&self, wrapper: T) -> R;
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