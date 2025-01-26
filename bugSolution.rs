fn main() {
    let mut v = vec![1, 2, 3];
    let first = v.get_mut(0);
    if let Some(first_element) = first {
        *first_element = 10;
        println!("First element: {}", v[0]);
    } else {
        println!("Could not get first element");
    }
} 