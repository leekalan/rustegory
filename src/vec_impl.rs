use std::borrow::Borrow;

use crate::{
    composer::{Composer, ComposerRef},
    wrapper::{Wrapper, WrapperRef},
};

impl<T> Wrapper<T, Vec<T>> for T {
    fn wrap(self, generic: T) -> Vec<T> {
        vec![self, generic]
    }
}

impl<T> Wrapper<Vec<T>, Vec<T>> for T {
    fn wrap(self, generic: Vec<T>) -> Vec<T> {
        let mut vec = generic;
        vec.insert(0, self);
        vec
    }
}

impl<T> Composer<T> for Vec<T> {
    fn compose(&mut self, generic: T) -> &mut Self {
        self.push(generic);
        self
    }
}

impl<T> Composer<Vec<T>> for Vec<T> {
    fn compose(&mut self, generic: Vec<T>) -> &mut Self {
        self.extend(generic);
        self
    }
}

impl<T: Copy> WrapperRef<T, Vec<T>> for T {
    fn wrap_ref(self, generic: impl Borrow<T>) -> Vec<T> {
        vec![self, *generic.borrow()]
    }
}

impl<T: Copy> WrapperRef<[T], Vec<T>> for T {
    fn wrap_ref(self, generic: impl Borrow<[T]>) -> Vec<T> {
        let vec = Vec::from(generic.borrow());
        self.wrap(vec)
    }
}

impl<T: Copy> ComposerRef<T> for Vec<T> {
    fn compose_ref(&mut self, generic: impl Borrow<T>) -> &mut Self {
        self.compose(*generic.borrow())
    }
}

impl<T: Copy> ComposerRef<[T]> for Vec<T> {
    fn compose_ref(&mut self, generic: impl Borrow<[T]>) -> &mut Self {
        self.extend(generic.borrow());
        self
    }
}
