#[derive(Debug)]
struct Node {
    element: u32,
    next: List,
}

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
        let list = List::Link(Box::new(Node {
            element: 1024,
            next: List::Empty,
        }));
        dbg!(list);
    }
}
