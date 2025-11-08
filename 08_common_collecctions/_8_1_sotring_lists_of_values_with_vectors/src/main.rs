fn main() {
    // --- vector 생성 ---
    let v: Vec<i32> = Vec::new();

    // --- 초깃값과 함께 생성하기 ---
    let v = vec![1, 2, 3]; // 기본 정수형인 i32로 생성

    // --- 벡터 업데이트 ---
    let mut v = Vec::new(); // 벡터지만 mut를 붙여야 하는 부분 주의

    // 타입 명시를  생략할 수 있는 이유는 push의 값으로 자료형이 추론되기 때문임
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // --- 벡터 요소 읽기 ---
    let v = vec![1, 2, 3, 4, 5];

    // 인덱스 접근법(참조자를 얻어옴)
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get 접근법(Option<&T>를 얻어옴)
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // 두 가지 접근법을 제공하는 이유:
    // 1. 인덱스 접근법: 잘못된 접근에 대해 패닉(예외)을 일으키고 싶은 경우
    // 2. get 접근법: 잘못된 접근을 방어하고 안전한 접근으로 제공하고 싶은 경우

    // --- 참조자를 얻어오므로, 가변&불변 참조 규칙이 적용된다 ---
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // let first: Option<&i32> = v.get(0);

    // v.push(6); // first가 사용중이므로 v에 쓰기 불가능!

    println!("The first element is: {:#?}", first);

    // --- 벡터 내부 값 반복하기 ---
    // 기본 for문 사용하기
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // 가변 참조자로 수정하기
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 역참조자로 값 얻기
    }

    // --- Vector에 열거형을 저장하기 ---
    // Vector에 열거형을 저장할 수 있는 점이 놀라웠다.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // --- Vector가 버려지면 내부 요소도 버려진다 ---
    {
        let v = vec![1, 2, 3, 4];
    } // 여기서 v가 스코프 밖으로 벗어나고 해제된다
}
