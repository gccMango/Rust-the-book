trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("점박이")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("멍멍이")
    }
}


fn main() {
    println!("강아지 이름은 {}",Dog::baby_name());
    println!("새끼 강아지 이름은 {}",<Dog as Animal>::baby_name());
    // < type name as trait name>::func(if mathod for receiver, argu..)
    // if associated function, just input the argument.
}
