#[derive(Debug)]
struct RectAngle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = RectAngle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels", area(rect1));
}

fn area(rectangle: RectAngle) -> u32 {
    rectangle.height * rectangle.width
}
