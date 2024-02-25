struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> Self {
        Self { head: None }
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
        let list = LinkedList::empty();
    }
}
