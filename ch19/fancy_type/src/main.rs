fn main() {
    //alias
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //long name type

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hello"));

    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    // fn returns_long_type()->Box< dyn Fn() + Send + 'static> {}

    //type alias
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hello"));
    
    fn takes_long_type(f: Thunk) {}
    // fn returns_long_type()-> Thunk {}
}
