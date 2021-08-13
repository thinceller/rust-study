fn main() {
    // immutable
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // mutable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = y + 1;
    println!("The value of y is: {}", y);

    // 足し算
    let _sum = 5 + 10;
    // 引き算
    let _difference = 95.5 - 4.3;
    // 掛け算
    let _product = 4 * 30;
    // 割り算
    let _quotient = 56.7 / 32.2;
    // 余り
    let _remainder = 43 % 5;

    // boolean
    let _t = true;

    // 文字型
    let _c = 'a';

    // 複合型
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let five_hundred = tup.0;
    println!("five_hundred: {}", five_hundred);

    let a = [1, 2, 3];
    let _first = a[0];
}
