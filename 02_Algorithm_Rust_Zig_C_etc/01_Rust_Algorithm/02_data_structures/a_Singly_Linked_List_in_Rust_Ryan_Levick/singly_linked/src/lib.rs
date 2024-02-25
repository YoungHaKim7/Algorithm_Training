struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: u32) {
        let _ = std::mem::replace(&mut self.head, None);
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
