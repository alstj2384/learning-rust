fn main() {
    // --- 패닉 직접 호출하기 ---
    // panic!("crash and burn");

    // --- panic! 백트레이스 사용하기 ---
    let v = vec![1, 2, 3];

    v[99]; // out of index panic!
}
