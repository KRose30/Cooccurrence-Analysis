use std::collections::HashSet;
use std::collections::HashMap;

pub(crate) struct Element<T> (T, usize);

struct Node<T> {

    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
    value: Element<T>,

}

pub(crate) struct HashedStack<T, K> {
    map: HashMap<K, Box<Node<T>>>,
    top: Option<Box<Node<T>>>,
    bottom: Option<Box<Node<T>>>,
}

impl<T, K> HashedStack<T, K> {
    pub(crate) fn new(_set: HashSet<K>) -> HashedStack<T, K> {
        let size = _set.len();
        let mut map: HashMap<K, Option<Box<Node<T>>>> = HashMap::new();

        let mut last: Box<Option<Node<T>>> = Box::new(None); // Stores previous node
        let mut bottom = None;

        for item in _set.iter() {
            match last.prev {
                Some(x) => {
                    let current: Node<T> = Node::new(Some(&last), None, Element(item, 0));
                    map.insert(item, &current);
                    last.next = Box::new(Some(current));
                    last = Box::new(Some(current));
                }
                // Marks bottom of stack
                None => {
                    last = Node::new(Some(&head), None, Element(item, 0));
                    map.insert(item, &last);
                    bottom = Some(&last)
                }
            }
        }

        let top = &last;

        match bottom {
            Some(node) => HashedStack { map, top, bottom: node },
            None => println!("No stack built"),
        }
    }

    fn move_to_top(&mut self, item: K, value: usize) {

    }
}

fn main() {

}
