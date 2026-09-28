pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(n) => {
                self.next = n.next.as_deref();
                Some(&n.data)
            }
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            None => None,
            Some(n) => {
                self.next = n.next.as_deref_mut();
                Some(&mut n.data)
            }
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, data: T) {
        let node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node))
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            None => None,
            Some(node) => Some(&node.data),
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.head.as_mut() {
            None => None,
            Some(node) => Some(&mut node.data),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}
