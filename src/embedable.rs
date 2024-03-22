use crate::composer::{Composer, ComposerRef};

pub trait Embedable<T> {
    fn embed(self, wrapper: &mut T) -> &mut T;
}

pub trait EmbedableRef<T> {
    fn embed_ref<'a>(&self, wrapper: &'a mut T) -> &'a mut T;
}

impl<T: Composer<U>, U> Embedable<T> for U {
    fn embed(self, wrapper: &mut T) -> &mut T {
        wrapper.compose(self)
    }
}

impl<T: ComposerRef<U>, U> EmbedableRef<T> for U {
    fn embed_ref<'a>(&self, wrapper: &'a mut T) -> &'a mut T {
        wrapper.compose_ref(self)
    }
}