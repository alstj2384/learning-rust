fn main() {
    // 변수는 불변이므로 아래 코드는 실행 불가능
    // let x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");

    // 변수를 가변으로 만들 수 있음
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // 상수는 항상 불변이므로 mut 사용 불가능
    // 타입은 반드시 명시되어야함( :u32)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // 섀도잉
    let shadow_x = 5;
    let shadow_x = shadow_x + 1;

    {
        let shadow_x = shadow_x * 2;
        println!("The value of x in the inner scope is: {shadow_x}");
    }

    println!("The value of x is: {shadow_x}")
}
