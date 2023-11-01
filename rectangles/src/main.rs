struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    // 使用单个变量
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // 使用元组重构
    let rect1 = (30, 50);
    println!("The area1 of the rectangle is {} square pixels.", area1(rect1));

    // 使用结构体重构：赋予更多意义
    let rect2 = Rectangle{
        width: 30,
        height: 50
    };
    println!("The area2 of the rectangle is {} square pixels.", area2(&rect2));

    // 使用方法
    let rect3 = Rectangle{
        width: 30,
        height: 50
    };
    println!("The area3 of the rectangle is {} square pixels.", rect3.area());
    if rect3.width() {
        println!("The rectangle has a nonzero width, it is {}.", rect3.width);
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
