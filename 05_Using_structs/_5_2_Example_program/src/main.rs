fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
}

// 하나의 사각형을 계산하는데 독립된 매개변수 2개를 받고 있다.
// (두 매개변수의 연관성이 명확하지 않다!)
fn area(width: u32, height: u32) -> u32 {
    width * height
}
