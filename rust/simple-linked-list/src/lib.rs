use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    length: usize,
}

struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: None,
        });

        self.push_node(new_node);
        self.length += 1;
    }

    fn push_node(&mut self, mut node: Box<Node<T>>){
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node|{
            self.length -= 1;
            node.data
        })
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>>{
        self.head.take().map(|mut node|{
            self.head = node.next.take();
            node
        })
    }
    

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    /// push 1 [1]
    /// push 2 [1, 2]
    /// push 3 [1, 2, 3] ---------> Stack (1, 2, 3)
    /// 
    /// pop() --> 3
    /// pop() --> 2
    /// pop() --> 1
    /// pop() --> None --------> None
    /// 
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();

        let mut cur_node = self.head;
        while let Some(mut node) = cur_node {
            result.push(node.data);
            cur_node = node.next.take();
        }

        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = SimpleLinkedList::new();
        
        for i in _iter {
            result.push(i);
        }

        result
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = Vec::new();

        let mut cur_node = self.head;

        while let Some(mut node) = cur_node {
            result.insert(0usize, node.data);
            cur_node = node.next.take();
        }
        result

        // let mut result = Vec::new();
        // while let Some(data) = self.pop() {
        //     result.push(data);
        // }
        // result.reverse();
        // result
    }
}
