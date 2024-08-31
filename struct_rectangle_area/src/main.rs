#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let square = Rectangle::square(3);

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("{}", rectangle.can_hold(&rectangle2));
    println!("{}", rectangle2.can_hold(&rectangle3));

    println!("{:#?}", rectangle);
}
