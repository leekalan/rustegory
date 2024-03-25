use std::collections::HashMap;

use crate::{composer::Composer, wrapper::Wrapper};

impl<T: Eq + std::hash::Hash> Composer<HashMap<T, T>> for HashMap<T, T> {
    fn compose(&mut self, generic: HashMap<T, T>) -> &mut Self {
        let mut contents = generic;
        for value in self.values_mut() {
            if let Some(value_o) = contents.remove(value) {
                *value = value_o;
            }
        }
        self.extend(contents);
        self
    }
}

impl<K: Eq + std::hash::Hash, V> Wrapper<HashMap<K, V>, HashMap<K, V>> for HashMap<K, K> {
    fn wrap(self, generic: HashMap<K, V>) -> HashMap<K, V> {
        let mut contents = generic;
        let mut new_hash = HashMap::with_capacity(contents.len());
        for (key, value) in self {
            if let Some(value_o) = contents.remove(&value) {
                new_hash.insert(key, value_o);
            }
        }
        new_hash.extend(contents);
        new_hash
    }
}