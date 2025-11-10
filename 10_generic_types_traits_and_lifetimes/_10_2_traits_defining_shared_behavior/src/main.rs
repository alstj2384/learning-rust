fn main() {
    // --- 트레이트 구현하기 ---
    // 상황: NewsArticle과 Tweet라는 서로 다른 형식의 기사에서 요약 정보를 얻어와야하는 상황

    // 트레이트 정의
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // NewsArticle의 Summary Trait.
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // NewsArticle의 정보를 종합해 반환한다.
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // Tweet의 정보를 종합해 반환한다.
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // Trait을 이용한 정보 반환하기
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // --- 기본 구현 ---
    pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // 기본 동작을 정할 수 있다.
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // NewsArticle은 Summary 트레이트를 구현하지 않았으므로 기본 동작("(Read more...)")을 출력한다.
    println!("New article available! {}", article.summarize());


    // --- 트레이트에 다른 메서드 추가 가능 ---
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    
        // summarize를 구현하지 않고, 아래만 구현해도
        impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        // 아래는 기본값 + summarize_author가 오버라이딩 된 값을 반환한다.
        println!("1 new tweet: {}", tweet.summarize());
        }
    }

    // --- 트레이트를 매개변수로 사용하기 ---
    // 아래와 같이 사용하면, Summary Trait를 구현한 객체만 받을 수 있다.
    // 내부적으로 Summary 트레이트를 활용한 기능을 추가할 수 있게 된다.
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // --- 트레이트 바운드를 여러 개 지정하기 ---
    pub fn notify(item: &(impl Summary + Display)) {}

    // --- where로 보기 좋게 정의하기 ---
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    }

    // --- 트레이트를 구현하는 타입을 반환하기 ---
    // 이 경우 반환 타입이 Tweet로 고정되어 있지만, 이 함수를 출력하는 입장에선 Tweet인지 몰라도 된다는 점을 주목하자
    fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }


    // --- 트레이드 바운드를 사용해 조건부로 메서드 구현하기 ---
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // 아래 메서드는 Display와 PartialOrd를 구현한 타입만 사용할 수 있다.
    // T가 i32인 경우, 기본적으로 둘 다 정의되어 있어 사용할 수 있다.
    // 커스텀 타입으로 이 메서드를 사용하고 싶은 경우, 나머지 두 트레이트를 구현하면 된다.
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
}
