use std::thread;
use std::time::Duration;

fn main() {
    // --- Thread 생성하기 ---
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 위 코드에서 메인 스레드의 반복문이 먼저 끝나면, 남은 스레드도 종료된다.

    // --- 모든 스레드가 끝날 때까지 기다리기 ---
    // spawn의 반환 타입은 JoinHandle이다. 이는 join 메서드 호출 시 스레드를 끝날때까지 기다리게 하는 소윳값임.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // --- 스레드에 move 클로저 사용하기(소유권 관련) ---
    // Rust의 소유권 시스템으로 스레드 호출 시에도 매개변수의 소유권 이전이 발생한다.

    // 아래 코드는 소유권 문제가 발생한다.
    // 새 스레드가 언제 실행될지 모르기 때문에, v가 언제 사라질지 모르기 때문이다.
    let v = vec![1, 2, 3];
    // thread::spawn(|| println!("{:?}", v));

    // 아래와 같이 move 크로저를 사용하면, 소유권을 가져간다.
    thread::spawn(move || {
        println!("{:?}", v);
    });

    // 따라서, 아래와 같이 v를 재참조할 수 없다.
    // v.get(0);
}
