use std::collections::HashMap;

fn main() {
    // --- 새로운 해시맵 생성하기 ---
    let mut scores = HashMap::new();

    // String, i32 타입으로 정의되는 Hash Map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);

    // --- 해시맵 값 접근하기 ---
    let team_name = String::from("Blue");
    // 1. team_name의 참조자로 정보 조회
    // 2. 반환된 Option<&V>를 copied()로 복사하여 Option<i32>로 얻어옴
    // 3. unwrap_or(0)으로 값이 없는 경우 0을 반환
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // --- 반복 작업 수행 ---
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // --- 해시맵 업데이트하기 ---

    // 값을 덮어쓰기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // 키가 없을 때만 키와 값 추가하기(기본 값 설정하기?)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 예전 값에 기초하여 값을 업데이트하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert는 가변 참조자(&mut V)를 반환하여 수정 가능함!
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
