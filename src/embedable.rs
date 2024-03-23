/*!
the semantic opposite of a composer, it is automatically implemented and is just the object and generic swapped

it is recomended to implement composer as embedable will then be automatically implemented

the 4 trait variants are as follows:

* [`Embedable`]
* [`EmbedableRef`]
* [`TryEmbedable`]
* [`TryEmbedableRef`]

*/

use std::borrow::BorrowMut;

use crate::composer::{Composer, ComposerRef, TryComposer, TryComposerRef};

pub trait Embedable<T> {
    fn embed(self, composer: &mut impl BorrowMut<T>) -> &mut T;
}

pub trait EmbedableRef<'a, T>: 'a {
    fn embed_ref<'b>(&'a self, composer: &'b mut impl BorrowMut<T>) -> &'b mut T;
}

pub trait TryEmbedable<T> {
    type Error;
    fn try_embed(self, composer: &mut impl BorrowMut<T>) -> Result<&mut T, Self::Error>;
}

pub trait TryEmbedableRef<'a, T>: 'a {
    type Error;
    fn try_embed_ref<'b>(
        &'a self,
        composer: &'b mut impl BorrowMut<T>,
    ) -> Result<&'b mut T, Self::Error>;
}

impl<T: Composer<U>, U> Embedable<T> for U {
    fn embed(self, composer: &mut impl BorrowMut<T>) -> &mut T {
        composer.borrow_mut().compose(self)
    }
}

impl<'a, T: ComposerRef<'a, U>, U: 'a> EmbedableRef<'a, T> for U {
    fn embed_ref<'b>(&'a self, composer: &'b mut impl BorrowMut<T>) -> &'b mut T {
        composer.borrow_mut().compose_ref(self)
    }
}

impl<T: TryComposer<U>, U> TryEmbedable<T> for U {
    type Error = T::Error;

    fn try_embed(self, composer: &mut impl BorrowMut<T>) -> Result<&mut T, Self::Error> {
        composer.borrow_mut().try_compose(self)
    }
}

impl<'a, T: TryComposerRef<'a, U>, U: 'a> TryEmbedableRef<'a, T> for U {
    type Error = T::Error;

    fn try_embed_ref<'b>(
        &'a self,
        composer: &'b mut impl BorrowMut<T>,
    ) -> Result<&'b mut T, Self::Error> {
        composer.borrow_mut().try_compose_ref(self)
    }
}
