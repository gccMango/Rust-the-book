extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("C에 따르면 -3의 절댓값은 {}입니다.",abs(-3));
    }
}
