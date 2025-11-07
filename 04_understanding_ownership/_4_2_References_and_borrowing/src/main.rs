fn main() {
    // 함수 호출 시 소유권이 이동하여 기존 함수에서 변수 사용이 불가능해지는 문제를 해결해보자.
    // --- 참조자를 이용할 수 있다. ---
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // --- 참조자가 가르키는 값은 수정할 수 없다 ---
    let s = String::from("hello");
    // change(&s); // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference

    // --- 가변 참조자를 이용하면 수정할 수 있다. ---
    let mut s = String::from("hello");
    change2(&mut s);
    println!("가변 참조자로 수정된 값: {}", s);

    // --- 한 변수에 가변 참조자는 한 개만 만들 수 있다. ---
    // 데이터 경합을 방지하기 위함
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time

    // println!("{}, {}", r1, r2);

    // --- 불변 참조자는 읽기 전용이므로 여러 개 생성 가능하다 ---
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);
    // 이 지점 이후로 r1, r2는 사용되지 않음.

    let r3 = &mut s; // r1, r2는 사용되지 않으므로 가변 참조자 가능.
    println!("{}", r3);

    // --- 댕글링 참조가 생성되지 않음을 컴파일러가 보장해준다 ---
    let reference_to_nothing = dangle(); // 실체없는 참조자가 반환된다.
}

fn calculate_length(s: &String) -> usize {
    // 64비트라면 64비트 부호 없는 정수형 반환
    s.len()
} // s가 스코프를 벗어남. 하지만 s1의 참조자이므로, 실제 값은 버려지지 않음

fn change(some_string: &String) {
    // some_string.push_str(", world");// error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // 참조를 반환한다.
} // s 는 이때 할당이 해제된다.
