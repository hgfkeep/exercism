use std::iter::FromIterator;

struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let node = Box::new(Node{data: _element, next: self.head.take()});
        self.head = Some(node);
        self.len +=1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            _ => {
                self.len -=1;
                //after take, type: Option<Box<Node<T>>>, replace origin Node
                self.head.take().map(|node|{
                    //node的类型为Box<Node<T>>, 由于map是闭包，解Optiona得到
                    //*node 为取node的地址，最终类型为Node<T>
                    let node = *node;
                    self.head  = node.next;
                    node.data
                })
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        //Option<&Box<Node<T>>>
        self.head.as_ref()
        //node 类型为&Box<Node<T>>, &node.data 类型为&T
        .map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rev_list: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut cur = self;
        while let Some(data) = cur.pop() {
            rev_list.push(data);
        }
        rev_list
    }
}

// impl<T: Clone> SimpleLinkedList<T> {
//     pub fn rev(self) -> SimpleLinkedList<T> {
//         let mut rev_list: SimpleLinkedList<T> = SimpleLinkedList::new();
//         //node的类型是&Box<Node<T>>
//         let mut next = self.head.as_ref();
//         while let Some(node) = next {
//             rev_list.push(node.data.clone());
//             next = node.next.as_ref();
//         }

//         rev_list
//     }
// }

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut linked_list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for i in _iter {
            linked_list.push(i);
        }

        linked_list
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
    fn into(mut self) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        while let Some(node) = self.pop(){
            res.insert(0, node);
        }
        res
    }
}
