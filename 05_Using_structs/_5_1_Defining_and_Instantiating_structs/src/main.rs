struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // --- 기본 구조체 생성 ---
    let user1 = User {
        active: true,
        username: String::from("minseo123"),
        email: String::from("myEmail@naver.com"),
        sign_in_count: 1,
    };

    // --- 가변 구조체로 선언하여 필드 변경하기 ---
    // 가변 여부는 전체 필드에 동일하게 적용됨.
    // 일부만 가변으로 만들지 못 함
    let mut user1 = User {
        active: true,
        username: String::from("minseo123"),
        email: String::from("myEmail@naver.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // --- 구조체 업데이트 문법 사용하기 ---
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1은 더 이상 사용 불가능.(user1.username의 이동으로 인해서)

    // --- 튜플 구조체 사용하기 ---
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Color와 Point는 매개변수가 같아도 다른 타입임.
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
