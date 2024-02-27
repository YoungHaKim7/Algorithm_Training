#[derive(Debug, PartialEq)]
pub struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T> Clone for DoublyLinkedList<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut new_list = DoublyLinkedList::<T>::new();
        if let Some(head_node) = &self.head {
            new_list.push(*head_node);
            let mut current_node = head_node;
            while let Some(next_node) = &current_node.next {
                new_list.push(*next_node);
                current_node = next_node;
            }
        }

        new_list
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            element,
            next: None,
            prev: None,
        };

        match &mut self.tail {
            Some(tail) => {
                tail.next = Some(Box::new(new_node));
                new_node.prev = Some(*tail);
                self.tail = Some(tail.clone());
            }
            None => {
                self.head = Some(Box::new(new_node));
                self.tail = Some(Box::new(new_node));
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|mut tail| {
            if let Some(mut prev_tail) = tail.prev.take() {
                prev_tail.next = None;
                self.tail = Some(prev_tail);
            } else {
                self.head = None;
            }

            tail.element
        })
    }

    pub fn peak(&self) -> Option<&T> {
        match &self.head {
            Some(head) => Some(&head.element),
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}
