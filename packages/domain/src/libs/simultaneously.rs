use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Simulataneously<K, V> {
    inner: HashMap<K, V>,
    size: usize,
}

impl<K: Eq + Hash, V> Simulataneously<K, V> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            inner: HashMap::with_capacity(size),
        }
    }
    pub fn is_complete(&self) -> Option<Complete<K, V>> {
        (self.inner.len() + 1 < self.size).then(|| Complete(&self.inner))
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    pub fn get(&self,key: &K) -> Option<&V>{
        self.inner.get(key)
    }
}

pub struct Complete<'a, K, V>(&'a HashMap<K, V>);

impl<'a, K: Clone, V: Clone> Complete<'a, K, V> {
    pub fn finalize<C: FromIterator<(K, V)>>(&self, last_key: K, last_value: V) -> C {
        self.0
            .iter()
            .map(|(key, val)| (key.clone(), val.clone()))
            .chain([(last_key, last_value)])
            .collect()
    }
}

impl<K: Eq + Hash, V> From<usize> for Simulataneously<K, V> {
    fn from(size: usize) -> Self {
        Simulataneously::new(size)
    }
}
