enum IpAddrKind {
    V4, // 배리언트
    V6, // 배리언트
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
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
}

fn route(ip_kind: IpAddrKind) {}
