use std::fs::File;
use std::io::{self, Read};

fn main() {
    // --- Result Enum을 반환하는 상황 ---
    // let greeting_file_result = File::open("hello.txt");
    // Ok 또는 Err 인스턴스가 반환된다.

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // --- 서로 다른 에러에 대해 매칭하기 ---
    // 위 예제는 실패 원인과 관련없이 panic!을 일으킨다.
    // 실패 이유에 따라 다른 조치를 취해보자.
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         // ErrorKind에 따라서 다른 요청 처리, NotFound이면 새로운 파일을 반환
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // --- 에러 발생 시 패닉을 빠르게 처리하는 방법 ---
    // Ok이면 값 반환, Err이면 panic! 호출
    // let greeting_file = File::open("hello.txt").unwrap();

    // expect로 panic! 에러 메시지 선택 가능.
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    // --- 에러 전파하기 ---
    // 에러 처리를 함수를 호출하는 쪽에서 할 수 있다(전파)
    // read_username_from_file().expect("custom panic!");

    // --- 에러 처리를 편리하게 해주는 <?> ---
    // Result나 Option에서 ? 연산자를 사용할 수 있다
    // let mut username = String::new();

    // let mut file = match File::open("hello.txt") {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };

    // match file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    // 위 코드를 짧게 줄여 쓰면 다음과 같다.
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
