// src\second.rs
use std::mem;

pub struct List {
    head: Link,
    count:i32,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty, count: 0 }
    }
    
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
    
        self.head = Link::More(new_node);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                self.count -= 1;
                Some(node.elem)
            }
        }
        
    }

    pub fn length(&mut self) -> i32 {
        self.count
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed node goes out of scope and gets dropped
            // it's next node has been set to link::empty
            // no unbounded recursion :)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Empty List
        assert_eq!(list.pop(), None);
        assert_eq!(list.length(), 0);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.length(), 3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.length(), 1);


        list.push(4);
        list.push(5);
        assert_eq!(list.length(), 3);

        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(),Some(4));
        assert_eq!(list.length(), 1);

        assert_eq!(list.pop(),Some(1));
        assert_eq!(list.length(), 0);
        assert_eq!(list.pop(),None);
        assert_eq!(list.length(), 0);
    }
}
