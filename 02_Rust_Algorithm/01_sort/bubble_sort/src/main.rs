fn print_array<T: std::fmt::Display>(array: &[T]) {
    print!("[");
    for (i, elem) in array.iter().enumerate() {
        print!("{}", elem);

        if i != array.len() - 1 {
            print!(",");
        }
    }
    println!("]");
}

fn main() {
    let mut sortable: [i32; 5] = [5, 8, 2, 7, 6];
    let length = sortable.len();
    let mut swapped = true;

    print_array(&sortable);

    while swapped {
        swapped = false;
        for i in 1..length {
            let previous_element = sortable[i - 1];
            let current_element = sortable[i];

            if previous_element > current_element {
                sortable.swap(i - 1, i);
                swapped = true;
            }
        }
    }

    print_array(&sortable);
}
