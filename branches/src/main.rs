fn main() {
    let number = 5;

    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number is equal to 5");
    } else {
        println!("number is greater than 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);
}
