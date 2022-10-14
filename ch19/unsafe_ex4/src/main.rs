//불변 정적 변수를 선언 및 사용
static HELLO_WORLD: &str = "안녕하세요!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe{
        COUNTER +=inc;
    }
}
fn main() {
    println!("인삿말: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe{
        println!("COUNTER: {}",COUNTER);
    }
}
