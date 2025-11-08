use crate::garden::vegetables::Asparagus;
// 패키지 경로를 use로 줄여서, Asparagus라고 간편히 호출 가능.

pub mod garden;
// 컴파일러에게 src/garden.rs에 있는 코드를 포함할 것을 알려줌

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
