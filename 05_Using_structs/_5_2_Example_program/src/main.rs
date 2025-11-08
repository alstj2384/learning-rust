fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

// 구조체로 받음으로써 각 필드명을 명확히 알 수 있게 되었다.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    width: u32,
    height: u32,
}
