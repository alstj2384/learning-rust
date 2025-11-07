fn main() {
    // 함수 사용
    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'h');

    // 함수는 표현식과 구문으로 나열된다.
    // 구문: 어떤 동작 수행 후 값을 반환하지 않음
    // 표현식: 결괏값을 평가함

    // 다음은 구문
    let y = 6;
    // 구문은 반환값이 없으므로 아래 표현은 불가능
    // let x = (let y = 6);

    // 표현식은 세미콜론을 쓰지 않음.
    // 표현식 끝에 세미콜론 추가 시 표현식은 구문이 됨
    let x = five();
    println!("The value of x is: {x}");
}

// 함수 정의
fn another_function() {
    println!("Another function.");
}

// 매개 변수를 받는 함수
// 반드시 매개변수 타입을 선언해야 한다.
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

// 여러 매개변수 정의하기
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 반환 값이 있는 함수
fn five() -> i32 {
    5 // 표현식
}
