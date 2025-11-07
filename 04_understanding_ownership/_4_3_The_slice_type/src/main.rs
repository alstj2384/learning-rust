fn main() {
    // --- 슬라이스는 컬렉션의 일부 요소를 참조할 수 있게 한다 (참조자의 일종) ---
    let s = String::from("hello world");
    let len = first_word(&s);
    println!("{}", len);

    // --- 문자열의 일부만 참조할 수 있다(파이썬의 슬라이스와 유사) ---
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // --- 일부 참조 개선 ---
    let s = String::from("hello world");
    let hello = first_word2(&s); // 원래 변수의 일부를 참조하는 슬라이스 참조자
    println!("문자열 s의 참조자: {}", hello);
    println!("원래 문자열 s도 여전히 유효하다: {}", s);

    // --- 문자열 슬라이스를 매개변수로 사용하기 ---
    let my_string = String::from("hello world");

    // first_word는 `String`의 일부 혹은 전체 슬라이스에 대해 작동한다.
    let word = first_word2(&my_string[0..6]);
    let word = first_word2(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의 참조자에 대해서도 작동한다.
    let word = first_word2(&my_string);

    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동한다.
    let word = first_word2(&my_string_literal[0..6]);
    let word = first_word2(&my_string_literal[..]);

    // 문자열 리터럴은 곧 문자열 슬라이스이므로, 아래의 코드도 슬라이스 문법 없이 작동한다.
    let word = first_word2(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
