struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: u32) {
        let new_head = Some(Box::new(Node {
            element,
            next: self.head.take(),
        }));
        self.head = new_head;
    }
}

#[derive(Debug)]
struct Node {
    element: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(1024);
    }
}
