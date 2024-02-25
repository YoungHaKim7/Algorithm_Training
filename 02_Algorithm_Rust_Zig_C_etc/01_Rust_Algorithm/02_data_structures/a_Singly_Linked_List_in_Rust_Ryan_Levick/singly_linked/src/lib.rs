struct LinkedList {
    head: Link,
}

impl Copy for LinkedList {}

impl LinkedList {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: u32) {
        match self.head {
            None => {
                self.head = Some(Box::new(Node {
                    element,
                    next: None,
                }))
            }
            Some(n) => {
                let new_head = Some(Box::new(Node {
                    element,
                    next: Some(n),
                }));
                self.head = new_head;
            }
        }
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
