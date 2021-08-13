fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("Goodbye!");

    let array = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("the value is: {}", element);
    }
}
