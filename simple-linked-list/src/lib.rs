use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        //current的类型是 &Option<Box<Node<T>>>
        let mut current = self.head.as_ref();

        //node的类型是 &Box<Node<T>>
        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }

        len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        //take()的结果：Option<Box<Node<T>>>
        match self.head.take() {
            None => None,
            //node的类型 Box<Node<T>>
            Some(mut node) => {
                self.head = node.next.take();
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            //node类型 &Box<Node<T>>, node.data 的类型，  &node.data的类型是&T
            Some(ref node) => {Some(&node.data)},
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut current = self;
        while let Some(data) = current.pop() {
            reversed.push(data);
        }

        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
        }

        list
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
        let mut vec = Vec::new();
        let mut current = self;
        while let Some(data) = current.pop() {
            vec.push(data);
        }
        vec.reverse();

        vec
    }
}
