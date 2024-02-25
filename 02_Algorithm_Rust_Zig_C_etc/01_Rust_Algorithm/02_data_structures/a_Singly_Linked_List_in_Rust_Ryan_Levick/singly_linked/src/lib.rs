struct LinkedList {
    head: Link,
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
        let list = LinkedList {
            head: Some(Box::new(Node {
                element: 1024,
                next: None,
            })),
        };
    }
}
