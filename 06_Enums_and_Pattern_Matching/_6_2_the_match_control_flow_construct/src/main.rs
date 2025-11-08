fn main() {
    // --- match는 매우 강력한 흐름 제어 연산자다 ---
    let u1 = value_in_cents(Coin::Penny);
    println!("penny = {}", u1);

    // --- match로 값을 바인딩하기 ---
    let u1 = value_in_cents2(CoinWithStateInformation::Quarter(UsState::Alaska));
    println!("quarter = {}", u1);

    // --- Option<T>를 이용하여 매칭하기 ---
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // --- Match를 사용하는 패턴은 모든 경우를 다루어야 한다 ---
    // 아래 코드는 Option의 모든 배리언트(Some, None)을 모두 다루지 않아 컴파일 오류가 발생한다.
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // --- 포괄 패턴으로 기본 동작 설정하기 ---
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // --- 매개변수를 사용하지 않는 경우 처리
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        //_ => (), 이렇게도 처리 가능
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // coin의 배리언트에 매칭되는 경우의 행동을 선언할 수 있다.
    match coin {
        Coin::Penny => 1, // Match Arm(갈래)
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --생략--
}

enum CoinWithStateInformation {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: CoinWithStateInformation) -> u8 {
    // coin의 배리언트에 매칭되는 경우의 행동을 선언할 수 있다.
    match coin {
        CoinWithStateInformation::Penny => 1, // Match Arm(갈래)
        CoinWithStateInformation::Nickel => 5,
        CoinWithStateInformation::Dime => 10,
        CoinWithStateInformation::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
