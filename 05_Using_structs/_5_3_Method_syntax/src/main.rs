#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 내부에 구현된 메서드를 연관 함수(association function)라고 한다.
impl Rectangle {
    // 메서드에서 값을 변경해야 한다면 &mut self로 가져올 수 있다.
    // 메서드는 첫 매개변수로 self를 가져야 한다.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    // getter 메서드는 필드 이름으로 작성한다.
    // 접근제어자를 이용해 캡슐화가 가능하다.
    fn height(&self) -> u32 {
        self.height
    }

    // 다른 사각형을 받아서 넓이를 비교한다.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 메서드가 아닌 연관함수 (생성자)
    fn square(size: u32) -> Self {
        // Self는 Rectangle을 의미한다.
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // --- 메서드는 구조체 컨텍스트에 정의되어야 한다 ---
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The value of height: {}", rect1.height());

    // --- 여러 매개변수를 받는 함수 ---
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // --- 연관 함수 호출 ---
    let sq = Rectangle::square(3);

    println!("The area of the rectangle is {} square pixels.", sq.area());
}
