use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --- 동시성을 보장하며 스레드간 데이터를 주고받기 ---
    // 메시지 패싱: 데이터를 담은 메시지를 주고 받으며 통신한다.

    // --- Channel 구현체 사용하기 ---
    // 채널은 한 스레드 -> 다른 스레드로 데이터를 보내는 일반적인 프로그래밍 개념이다.

    // 송신자와 수신자로 나뉨.
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // tx(송신 단말) 사용하기
    // thread::spawn(move || {
    //     // move로 tx를 전송시켜, 소유권을 스레드가 갖게 한다.
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // rx로 val 값을 보낸다.
    //                            // send시 단말이 닫힌 경우, Result는 Err를 반환한다.
    //                            // 해당 코드는 unwrap으로 처리했지만, 적절한 조치가 필요하다.
    // });

    // recv 수행 시, tx의 값이 전송될때까지 기다린다(Block)
    // tx 채널이 닫히면, 더 이상 값이 올 수 없으므로 에러를 반환한다.
    // let received = rx.recv().unwrap();

    // try_recv는 기다리지 않고 즉시 결과를 반환한다.
    // 메시지가 있다면 Ok, 없다면 Err 반환.
    // let received = rx.try_recv();
    // println!("Got: {}", received);

    // --- 여러 값 보내기와 수신자가 기다리는지 알아보기 ---
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // rx를 반복자처럼 다루며, 응답을 처리한다.
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // --- 여러 생산자 만들기 ---
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
