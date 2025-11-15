fn main() {
    // Box는 데이터를 힙에 저장하고, 스택에서는 힙을 가르키는 포인터를 보관한다.

    // --- Box<T>를 사용하여 힙에 데이터 저장하기 ---
    let b = Box::new(5);
    // b는 힙에 저장된다. 따라서, 이동될 경우 사용할 수 없어진다.
    // let a = b;
    println!("b = {}", b);

    // --- Vec 내부 요소의 연속성 확인하기 ---
    let mut v = vec![1, 2, 3, 4, 5];
    for i in 0..v.len() {
        println!("{:p}", &v[i] as *const i32);
    }
    // 0x136e06100
    // 0x136e06104
    // 0x136e06108
    // 0x136e0610c
    // 0x136e06110
    // 모두 4바이트 단위로 연속된 공간에 위치한다.

    println!();

    // --- 만약, Vec<Box>>로 구성한다면? ---
    let mut v = vec![
        Box::new(1),
        Box::new(2),
        Box::new(3),
        Box::new(4),
        Box::new(5),
    ];
    for i in 0..v.len() {
        println!("{:p}", &v[i]);
    }
    // 0x152605f70
    // 0x152605f78
    // 0x152605f80
    // 0x152605f88
    // 0x152605f90
    // 8바이트 단위로 연속된다. -> Box가 8바이트 주소를 담기 때문?
    // Box 내부의 변수값들은 어디에 저장될까?

    println!("Boxing된 변수의 메모리 주소");
    for i in 0..v.len() {
        println!("{:p}", *(&v[i]));
    }
}
