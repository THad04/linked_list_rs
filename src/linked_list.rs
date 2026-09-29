pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

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
    next: Option<&'a mut Node<T>>,
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
        LinkedList { head: None, len: 0 }
    }

    pub fn push(&mut self, data: T) {
        let node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            None => None,
            Some(node) => {
                self.head = node.next;
                self.len -= 1;
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

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_works() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        if let Some(value) = list.peek_mut() {
            *value = 42;
        }
        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn iterate_linked_list() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let sum: i32 = list.into_iter().sum();
        assert_eq!(sum, 6);

        let mut list_2 = LinkedList::new();
        list_2.push(1);
        list_2.push(2);
        list_2.push(3);
        list_2.iter_mut().for_each(|x: &mut i32| *x *= 2);
        assert_eq!(list_2.pop(), Some(6));
        assert_eq!(list_2.pop(), Some(4));
        assert_eq!(list_2.pop(), Some(2));
    }

    #[test]
    fn reference_loop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        for value in &list {
            println!("{}", value);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        for value in &mut list {
            *value *= 2;
        }
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(2));
    }
}
