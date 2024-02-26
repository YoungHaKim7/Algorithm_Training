#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl LinkedList<u32> {}
impl LinkedList<String> {}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
    }

    pub fn peak(&mut self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(1);
        list.pop();
        list.peak();
    }

    #[test]
    fn list_push() {
        let mut list = LinkedList::empty();
        list.push(2000);
    }

    #[test]
    fn list_pop() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(2);
        list.pop();
    }

    #[test]
    fn list_peak() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(0);
        list.peak();
    }

    #[test]
    fn push_test() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(1);

        assert_eq!(
            list,
            LinkedList {
                head: Some(Box::new(Node {
                    element: 1,
                    next: Some(Box::new(Node {
                        element: 1024,
                        next: None,
                    }),),
                }),),
            }
        );
    }
}
