/*!
# Wrappr
[`wrappr`](self) is a libary for some useful wrapping traits

the 4 trait groups are as follows:

* a [`wrapper`] is a cheap wrapper that constructs a new type, generic over the input type
* a [`composer`] is a potentially costly operation that takes a generic input type, and composes it within itself
* a [`injectable`] is the semantic opposite of a wrapper, it is automatically implemented and is just the object and generic swapped
* a [`embedable`] is the semantic opposite of a composer, it is automatically implemented and is just the object and generic swapped

# Examples

## Wrapper

```rust
# use wrappr::wrapper::Wrapper;
let x: u8 = 1;
let y: u8 = 2;
let z: Vec<u8> = vec![3, 4];

let yz: Vec<u8> = y.wrap(z);
assert_eq!(yz, vec![2, 3, 4]);

let xyz: Vec<u8> = x.wrap(yz);
assert_eq!(xyz, vec![1, 2, 3, 4]);
```

## Composer

```rust
# use wrappr::composer::Composer;
let mut x: Vec<u8> = vec![1, 2];
let y: Vec<u8> = vec![3, 4];
let z: u8 = 5;

x.compose(y);
assert_eq!(x, vec![1, 2, 3, 4]);

x.compose(z);
assert_eq!(x, vec![1, 2, 3, 4, 5]);
```

## Injectable

```rust
# use wrappr::injectable::Injectable;
let x: u8 = 1;
let y: u8 = 2;
let z: Vec<u8> = vec![3, 4];

let yz: Vec<u8> = z.inject(y);
assert_eq!(yz, vec![2, 3, 4]);

let xyz: Vec<u8> = yz.inject(x);
assert_eq!(xyz, vec![1, 2, 3, 4]);
```

## Embedable

```rust
# use wrappr::embedable::Embedable;
let mut x: Vec<u8> = vec![1, 2];
let y: Vec<u8> = vec![3, 4];
let z: u8 = 5;

y.embed(&mut x);
assert_eq!(x, vec![1, 2, 3, 4]);

z.embed(&mut x);
assert_eq!(x, vec![1, 2, 3, 4, 5]);
```

*/

#![allow(unused)]

pub mod composer;
pub mod embedable;
pub mod injectable;
/// implementations for vectors
pub mod vec_impl;
pub mod wrapper;

#[cfg(test)]
mod tests {
    use self::wrapper::WrapperRef;

    use super::*;

    use std::borrow::Borrow;

    struct RefImpl<'a, T>(&'a T);

    impl<'a, T> wrapper::WrapperRef<'a, T, RefImpl<'a, T>> for () {
        fn wrap_ref(self, generic: &'a impl Borrow<T>) -> RefImpl<'a, T> {
            RefImpl(generic.borrow())
        }
    }

    impl<'a, T> composer::ComposerRef<'a, T> for RefImpl<'a, T> {
        fn compose_ref(&mut self, generic: &'a impl Borrow<T>) -> &mut Self {
            self.0 = generic.borrow();
            self
        }
    }
}
