
struct Node<K, V>
  where K: Ord + Sized, V: Sized {
    left : Option<Box<Node<K, V>>>,
    right : Option<Box<Node<K, V>>>,
    key: K,
    value: V,
}

pub struct Tree<K, V>
  where K: Ord + Sized, V: Sized {
    root: Option<Node<K, V>>,
}

impl <K, V> Tree<K, V>
  where K: Ord + Sized, V: Sized {
    // fn insert(&mut self, key: K, value: V) -> Option<V> {
    fn insert(&mut self, key: K, value: V) {
    }

    fn get(&self, key: &K) -> Option<&V> {
        None
    }
}
