/*!
the semantic opposite of a composer, it is automatically implemented and is just the object and generic swapped

it is recomended to implement composer as embedable will then be automatically implemented

the 4 trait variants are as follows:

* [`Embedable`]
* [`EmbedableRef`]
* [`TryEmbedable`]
* [`TryEmbedableRef`]

*/

use crate::composer::{Composer, ComposerRef, TryComposer, TryComposerRef};

pub trait Embedable<T> {
    fn embed(self, composer: &mut T) -> &mut T;
}

pub trait EmbedableRef<T> {
    fn embed_ref<'a>(&self, composer: &'a mut T) -> &'a mut T;
}

pub trait TryEmbedable<T> {
    type Error;
    fn try_embed(self, composer: &mut T) -> Result<&mut T, Self::Error>;
}

pub trait TryEmbedableRef<T> {
    type Error;
    fn try_embed_ref<'a>(&self, composer: &'a mut T) -> Result<&'a mut T, Self::Error>;
}

impl<T: Composer<U>, U> Embedable<T> for U {
    fn embed(self, composer: &mut T) -> &mut T {
        composer.compose(self)
    }
}

impl<T: ComposerRef<U>, U> EmbedableRef<T> for U {
    fn embed_ref<'a>(&self, composer: &'a mut T) -> &'a mut T {
        composer.compose_ref(self)
    }
}

impl<T: TryComposer<U>, U> TryEmbedable<T> for U {
    type Error = T::Error;

    fn try_embed(self, composer: &mut T) -> Result<&mut T, Self::Error> {
        composer.try_compose(self)
    }
}

impl<T: TryComposerRef<U>, U> TryEmbedableRef<T> for U {
    type Error = T::Error;

    fn try_embed_ref<'a>(&self, composer: &'a mut T) -> Result<&'a mut T, Self::Error> {
        composer.try_compose_ref(self)
    }
}
