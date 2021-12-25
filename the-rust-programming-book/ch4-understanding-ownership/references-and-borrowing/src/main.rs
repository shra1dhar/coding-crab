fn main() {
    let s1 = String::from("Femo challenge");
    let len = calculate_length(&s1);
    println!("Size of strign {} is {}", s1, len)
}

// For borrowing, we'll pass String by reference here
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}
