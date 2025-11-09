fn main() {

    // --- 하나의 패턴만 매칭하고 싶은 경우 ---

    // 아래 코드는 하나의 패턴 매칭을 위해 보일러 플레이트 코드가 생긴다.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // if let 문법을 이용하여 더 짧게 표현할 수 있다.
    // 단, match의 철저한 검사를 하지 않게 되는 부분은 고려해야 한다.
        let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}
