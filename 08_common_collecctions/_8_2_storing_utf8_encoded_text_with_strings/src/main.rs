fn main() {
    // --- 빈 문자열 생성하기 ---
    let mut s = String::new();

    // --- 문자열 리터럴로부터 String 생성하기 ---
    let data = "initial contents";

    let s = data.to_string();

    // 리터럴을 바로 String으로 바꿀 수 있다.
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    // to_string과 from은 스타일과 가독성의 차이!

    // --- 문자열은 UTF-8로 인코딩되어 아래 데이터와 같이 폭넓게 집어넣을 수 있다---
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // --- 문자열 업데이트하기---
    let mut s = String::from("foo");
    s.push_str("bar");

    // --- 문자열 슬라이스를 String에 이어 붙이고 문자열 슬라이스 사용하기 ---
    let mut s1 = String::from("foo");
    let s2 = "bar"; // 문자열슬라이스(&str)
    s1.push_str(s2);
    println!("s2 is {s2}"); // push_str에서 s2의 소유권을 가져가지 않는다.

    // --- push를 사용하여 String 값에 한 글자 추가하기 ---
    let mut s = String::from("lo");
    s.push('l');

    // --- + 연산자나 format! 매크로를 이용해 문자열 합치기 ---
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기로 이동되어 더 이상 사용할 수 없음!

    // --- &String이 &str로 변환될 수 있던 이유---
    // fn add(self, s: &str) -> String {
    // + 연산자는 내부적으로 add 메서드를 사용한다.
    // 이때, 러스트는 역참조 강제 변환을 통해 &str로 변환한다.

    // --- format!을 쓰자! ---
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // 매개변수 소유권도 가져가지 않는다.

    // --- 문자열 내부의 인덱싱 ---
    // UTF-8로 구성된 문자열은 여러 언어를 지원한다.
    // 이때, 각 언어의 바이트 수는 각기 다르다.
    // 1글자에 1바이트일수도, 2바이트일 수도 있다.
    // String은 vector<u8>로 구현되어있어, 한 칸에 1바이트를 의미한다.
    // 즉, 2바이트가 필요한 문자는 vector 2칸에 거쳐 저장된다는 의미임
    // 이로 인해 2바이트가 필요한 문자를 1바이트로 자르면 의도치 않은 결과가 발생한다
    // 이를 방지하기 위해 러스트에서는 문자열 인덱싱을 막아놨다.
    let hello = "Здравствуйте";
    let answer = &hello[0]; // 2바이트가 필요한 문자에 1바이트만을 얻어옴 -> 패닉

    // --- 문자열을 슬라이싱 한다면, 바이트 크기를 고려하자 ---
    // 러스트 자체적으로 슬라이싱을 지원하긴 하지만, 권장하진 않는다.
    // 개발자가 바이트 크기를 정확히 모르는 상태에서 사용하며 버그가 발생할 수 있기 때문임.
    // 따라서 잘못된 크기로 슬라이스를 하면 패닉을 일으켜서 버그를 방지한다.
    let hello = "Здравствуйте"; // 문자당 2바이트 단위의 언어
    let s = &hello[0..4]; // 2바이트 단위이므로 가능
                          // let s = &hello[0..1]; // 패닉!

    // --- 문자열에 대한 반복을 위한 메서드 ---
    // 문자열 바이트 크기를 고려하지 않고 조회하고 싶다면 chars()를 사용하면 된다.
    for c in "Зд".chars() {
        println!("{c}");
    }

    // 바이트 단위로도 표현할 수 있다.
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
