use std::os::unix::fs::OpenOptionsExt;

struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: u32) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<u32> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
    }

    fn peak(&mut self) -> Option<&u32> {
        self.head.as_ref().map(|n| &n.element)
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
        list.push(0);
        list.pop();
        list.peak();
    }
}
