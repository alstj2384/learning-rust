fn main() {
    // --- 제네릭 적용하기 ---
    // 함수, 구조체, 열거형, 메서드를 제네릭으로 정의할 수 있다.

    // 아래와 같이 서로 다른 자료형에서 가장 큰 값을 찾는다고 해보자.
    // 아래 두 함수는 완벽하게 동일하니, 제네릭을 이용해 하나의 함수로 만들어보자.
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 타입 이름을 지어줄 때는 카멜 케이스를 따른다.
    fn largest<T>(list: &[T]) -> &T { // T 타입의 슬라이스를 받고 반환한다.
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 아래와 같이 사용할 수 있다.
    // 다만, PartialOrd 관련 오류가 발생하는데, 이는 T가 될 수 있는 모든 타입에 대해
    // 함수 내부 동작이 정상적이지 않다는 것을 의미한다.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // --- 제네릭 구조체 정의 ---
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point(x: 5, y: 10);
    let float = Point{x: 1.0, y: 4.0};
    

    // T 타입은 동일해야 한다.
    // let wont_work = Point { x: 5, y: 4.0 };

    // --- 제네릭 열거형 정의 ---
    enum Option<T> { // Option 열거형
        Some(T), 
        None,
    }

    enum Result<T, E> { // 여러 개 제네릭을 사용 가능
        Ok(T), 
        Err(E),
    }

    // --- 제네릭 메서드 정의 ---
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> { // impl 옆 <T>를 주의.
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> { // 이렇게 하면 제네릭이 f32인 경우에만 사용 가능함.
        fn x(&self) -> &T {
            &self.x
        }
    }

}
