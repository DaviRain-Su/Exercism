use std::iter::FromIterator;

/// In the Box formulation of the linked list, each node owns its successor if there is one.
/// Since Box does not allow cloning the way Rc does, we need some way to work with node "pointers"
/// without taking ownership of them.  We'll do this by taking advantage of Option<T>'s `as_ref()`
/// and `as_mut()` methods, which return an `Option<&T>` and an `Option<&mut T>`,
/// respectively.  We also use the `take()` method to transfer ownership of objects in and out of
/// `Option`s.

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl <T> Node<T> {
   pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}


type Link<T> = Option<Box<Node<T>>>;

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut cur_node : Option<&Box<Node<T>>> = self.head.as_ref();
        while let Some(node_ref) = cur_node {
            count += 1;
            cur_node = node_ref.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let new_node : Box<Node<T>> = Box::new(Node::new(element));
        self.push_node(new_node);
    }

    fn push_node(&mut self, mut node: Box<Node<T>>){
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node|{
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

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node | {
            &mut node.data
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

        let mut cur_node : Option<Box<Node<T>>> = self.head;
        while let Some(mut node) = cur_node {
            result.push(node.data);
            cur_node = node.next.take();
        }

        result
    }
}


impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = SimpleLinkedList::new();
        
        for i in iter {
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

        let mut cur_node : Option<Box<Node<T>>> = self.head;

        while let Some(mut node) = cur_node {
            result.insert(0usize, node.data);
            cur_node = node.next.take();
        }

        result
    }
}

pub struct IntoIter<T>(SimpleLinkedList<T>);

impl <T> SimpleLinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>
}

impl <T> SimpleLinkedList<T> {
    pub fn iter(&self) -> Iter<'_,T> {
        Iter{ next: self.head.as_ref().map(|node|{
            & **node
        })}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node|{
            self.next = node.next.as_ref().map(|node| & **node);
            &node.data
        })
    }
}

pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>
}

impl <T> SimpleLinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut{
            next: self.head.as_mut().map(|node|{
                &mut **node
            })
        }
    }  
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_mut().map(|node| { &mut **node});
            &mut node.data
        })
    }
}
