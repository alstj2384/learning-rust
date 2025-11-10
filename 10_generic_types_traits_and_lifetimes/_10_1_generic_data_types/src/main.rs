fn main() {

    // --- Vec 내부 가장 큰 값을 구하는 프로그램 ---
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // --- 만약 하나가 더 추가되어야 한다면? ---
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    // 중복이 발생하고, 에러가 발생할 가능성이 높다. 그리고 응집도가 약해 여러 군데를 수정해야 한다.

    // --- 큰 수를 찾는 코드를 함수로 추출 ---
    fn largest(list: &[i32]) -> &32{
        let mut largest = &list[0];

        for item in list{
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    // 아래와 같이 간단히 호출할 수 있다.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // --- 제네릭 적용하기 ---
    // 함수, 구조체, 열거형, 메서드를 제네릭으로 정의할 수 있다.
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

}
