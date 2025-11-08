fn main() {
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area(rect1));
}

// 하나의 튜플로 받아 매개변수 의미가 명확해짐.
// 단, 여전히 인덱스로 표현되어 각 값이 무엇인지 뚜렷히 알기 어려움(높이? 너비?)
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
