#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// add functions of Rectangle inside impl block
impl Rectangle {
    // this is a struct method
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    // check that the given rectangle can be placed inside this rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// each struct can has multiple impl block
impl Rectangle {
    // this is a associated function
    // associated function must be called in this way: StructName::func_name()
    fn new(w: u32, h: u32) -> Rectangle {
        return Rectangle {
            width: w,
            height: h,
        };
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 32,
        width: 44,
    };

    let area = rect1.area();
    println!("the area is: {}", area);

    let rect2 = Rectangle {
        height: 21,
        width: 12,
    };

    let can_hold = rect1.can_hold(&rect2);
    println!("rect1 can hold rect2? {}", can_hold);

    // calling associated function
    let rect3 = Rectangle::new(10, 30);
    println!("rect3: {:?}", rect3);
}
