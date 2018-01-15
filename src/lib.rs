
struct Node<K, V>
  where K: Ord + Sized, V: Sized {
    left : Option<Box<Node<K, V>>>,
    right : Option<Box<Node<K, V>>>,
    key: K,
    value: V,
}

impl <K, V> Node<K, V>
  where K: Ord + Sized, V: Sized {

    fn new_leaf(key: K, value: V) -> Node<K, V> {
        Node{
            key: key,
            value: value,
            left: None,
            right: None
        }
    }
}

#[derive(Default)]
pub struct Tree<K, V>
  where K: Ord + Sized, V: Sized {
    root: Option<Node<K, V>>,
}

impl <K, V> Tree<K, V>
  where K: Ord + Sized, V: Sized {
    // fn insert(&mut self, key: K, value: V) -> Option<V> {
    pub fn insert(&mut self, key: K, value: V) {
        match self.root {
            None => self.root = Some(Node::new_leaf(key, value)),
            Some(_) => {}
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.root {
            Some(ref n) if &n.key == key => Some(&n.value),
            _ => None,
        }
    }
}
