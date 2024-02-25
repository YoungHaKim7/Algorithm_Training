struct LinkedList {
    head: Link,
}

#[derive(Debug)]
struct Node {
    element: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
enum List {
    Empty,
    Link(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = Some(Box::new(Node {
            element: 1024,
            next: None,
        }));
        dbg!(list);
    }
}
