fn main() {
    let s = String::from("Hello");

    takes_ownership(s);
    // takes_ownership(s);

    let s = String::from("hello");
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
