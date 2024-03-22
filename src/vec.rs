use crate::{composer::Composer, wrapper::Wrapper};

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

impl<T> Wrapper<Vec<T>> for T {
    type Result = Vec<T>;

    fn wrap(self, generic: Vec<T>) -> Self::Result {
        let mut vec = generic;
        vec.insert(0, self);
        vec
    }
}