mod linked_list;
use linked_list::LinkedList;

fn main() {}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_and_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        println!("{:?}", list.pop());
        println!("{:?}", list.pop());
        println!("{:?}", list.pop());
        println!("{:?}", list.pop());
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
}
