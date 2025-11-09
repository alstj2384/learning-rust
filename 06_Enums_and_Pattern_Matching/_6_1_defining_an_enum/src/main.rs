enum IpAddrKind {
    V4, // 배리언트
    V6, // 배리언트
}

enum IpAddrKindWithString {
    V4(String),
    V6(String), 
}

enum IpAddrKindIndependantFields {
    V4(u8, u8, u8, u8), 
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKindWithStruct {
    V4(IpAddr),
    V6(IpAddr),
}



fn main() {
    // --- 열거형은 여러 개의 가능한 값의 집합 중 하나를 나타낸다 ---
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // --- 배리언트 중 하나로 함수를 호출할 수 있다 ---
    route(IpAddrKind::V4);

    // --- 구조체의 필드로 Enum을 사용할 수 있다 ---
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // --- 베리언트 자체에 값을 포함시킬 수 있다 ---
    let home = IpAddrKindWithString::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindWithString::V6(String::from("::1"));

    // --- 배리언트는 각자 다른 필드를 가질 수 있다 ---
    let home = IpAddrKindIndependantFields::V4(127, 0, 0, 1);
    let loopback = IpAddrKindIndependantFields::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
