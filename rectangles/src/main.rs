#[derive(Debug)]
struct RectAngle {
    width: u32,
    height: u32,
}

impl RectAngle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &RectAngle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = RectAngle {
        width: 30,
        height: 50,
    };
    let rect2 = RectAngle {
        width: 10,
        height: 40,
    };

    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
