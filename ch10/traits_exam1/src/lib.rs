pub mod tweet {
    // use std::fmt::Display;

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub trait Summary {
        fn summarize(&self) -> String{
            format!("{}님의 기사 더 읽기", self.summarize_author())
        }

        fn summarize_author(&self) -> String;
    }

    impl Summary for NewsArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {}, ({})",self.headline,self.author,self.location)
        // }
        fn summarize_author(&self) -> String {
            format!("{}",self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        // fn summarize(&self) -> String {
        //     format!("{}: {}",self.username,self.content)
        // }
    }
    // impl trait
    // pub fn notify(item: impl Summar + Display) {
    //     println!("속보!! {}",item.summarize());
    // }
    
    // trait bound
    // pub fn notify<T:Summary + Display> (item: T) {
    //     println!("속보!! {}",item.summarize());
    // }
    pub fn notify<T:Summary> (item: T) {
        println!("속보!! {}",item.summarize());
    }
    // trait를 구현하는 값 리턴하기
    pub fn returns_summerizable() -> impl Summary {
        Tweet {
            username: String::from("tic tac toe"),
            content: String::from("I'm from library to main"),
            reply: false,
            retweet: false,
        }
    }
}

