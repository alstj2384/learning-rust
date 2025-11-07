fn main() {
    // 변수의 스코프
    {
        let s = "hello";
    } // 스코프 종료로 s가 더 이상 유효하지 않게 된다.

    // String 타입
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    } // drop 호출(컴파일러가 자동으로 s의 문자열 메모리를 해제한다)

    // 데이터 이동
    let x = 5;
    let y = x; // x는 여전히 유효하다(스택에 할당되므로)

    let s1 = String::from("hello");
    let s2 = s1; // s1은 더 이상  유효하지 않다

    //println!("{}", s1); // 오류 발생

    // 데이터 복사
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 깊은 복사(힙 영역까지 복사함)

    println!("s1 = {}, s2 = {}", s1, s2);

    // 소유권과 함수
    let s = String::from("hello");
    takes_ownership(s); // s의 소유권이 함수로 이동된다.

    // println!("{}", s); // error[E0382]: borrow of moved value: `s`

    let x = 5;
    makes_copy(x); // x는 복사되므로, 이후에도 사용할 수 있다.

    println!("{}", x);

    let s1 = gives_ownership(); // 함수에서 만들어진 소유권을 받는다.
    println!("함수로부터 소유권을 받은 문자열: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // 소유권을 주고, 다시 받는다.

    //println!("s2는 더 이상 유효하지 않다: {}", s2); // error[E0382]: borrow of moved value: `s2`
    println!("함수로 여행갔다온 문자열 s2: {}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 이 부분에서 some_string이 drop된다

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 소유권을 다시 반환한다.
}
