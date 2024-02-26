use singly_linked02::LinkedList;

fn main() {
    let mut list = LinkedList::empty();
    list.push(1024);
    list.push(1);
    dbg!(list);

    let mut list02 = LinkedList::empty();
    list02.push(2048);
    list02.push(256);
    dbg!(list02.peak());
    dbg!(list02);
}
