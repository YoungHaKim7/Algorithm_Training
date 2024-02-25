// struct LinkedList {
//     head: Link,
// }
//
// struct Node {
//     element: u32,
//     next: List,
// }
enum Node {
    Empty,
    NonEmpty(u32, &'static Node),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let list = List::NonEmpty(1091, Box::new(Node) {
        //     element: 1024,
        //     next: List::Empty,
        // });
        let list = Node::NonEmpty(1091, &(Node::Empty));
    }
}
