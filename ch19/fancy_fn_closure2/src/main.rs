fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x|x+1)
}

fn main() {
    let return_closure = returns_closure();
    println!("return closure input : 1 => result : {} ",return_closure(1));
}
