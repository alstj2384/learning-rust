fn main() {
    // 러스트는 정적 타입 언어. 컴파일 시점에 모든 변수가 정해져야 한다.

    // 스칼라 타입(정수, 부동 소수점 숫자, 부울린, 문자)

    // 정수형(i는 부호 있음, u는 부호 없음, 숫자는 비트 길이)
    let value: u8 = 255; // 가능

    // let value: u8 = 256; // 불가능 (범위 넘어섬)
    // let value: u8 = -5; // 불가능 (부호 없는 변수)

    println!("Dicimal: {}", 98_222);
    println!("Hex: {}", 0xff);
    println!("Octal: {}", 0o77);
    println!("Binary: {}", 0b1111_0000);
    println!("Byte: {}", b'A');

    // 부동 소수점 타입
    let x = 2.0; // f64(기본 할당)
    let y: f32 = 3.0; // f32

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // get tuple
    let (x, y, z) = tup;

    // array
    // 고정된 개수의 요소를 보관하는 경우 유용함
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 3으로 5개의 요소 초기화
    let a = [3; 5];

    // 배열 접근
    let first = a[0];
    let second = a[1];
}
