//dynamically sized type
// unsized type

use std::mem;

fn main() {
    // 컴파일 안됨. 메모리 할당에 대해 잘 모름
    // 특정 타입에 얼마나 메모리를 할당해야 할지
    // 특정 타입 개별당 반드시 같은 양의 메모리 공간을 차지 해야 한다.

    // let s1: str = "안녕하세요";
    // let s2: str = "잘지내나요?";
    // s2가 메모리공간을 더 차지한다. 같은 타입인데

    // 타입을 &str로 선언하면 간단하게 해결
    let s1: &str = "안녕하세요 abcdefg";
    let s2: &str = "잘지내나요? how are you?";
 
    println!("s1 memory size: {}",mem::size_of_val(s1));
    println!("s2 memory size: {}",mem::size_of_val(s2));
    println!();
    // 문자열 slice에서 slice의 구조는 시작위치(메모리 주소)와
    // 슬라이스의 길이를 저장 하고 있다.
    // &T는 &str과 달리 할당된 메모리 주소 하나의 값만을 갖는다.
    // &str은 주소와 길이 두개의 값을 갖는 구조이다.
    // 즉 64bit 기준 8byte 두개 값인 16byte가 &str의 메모리 크기이다.
    println!("s1 &str memory size: {:?}",mem::size_of::<&str>());
    println!("s2 &str memory size: {:?}",mem::size_of::<&str>());

    // 동적인 정보의 크기를 메타 데이터로 추가로 저장 한다.
    // 동적크기의 타입은 반드시 어떤포인터(Rc, Box 등등)으로 가리켜야 한다.

    // 모든 트레이트는 트레이트의 이름을 통해 참조할 수 있는 동적크기 타입
    // DST를 다루기 위해
    // Rust는 Sized 트레이트를 지원
    // 컴파일 타임에 타입의 크기를 알 수 있는 지 결정하는 데 사용
    // 암묵적으로 모든 제네릭 함수에 Sized 경계를 추가한다.
    // 크기가 알려진 모든 타입에는 자동 구현된다.
    fn generic<T>(t: T) {}
    // ⬇
    fn generic_real<T:Sized>(t:T){}

    // 특별 문법
    fn generic_special<T:?Sized>(t: &T) {}
    // T는 Sized 트레이트를 구현할 수도 아닐수도 있다.
    // 이 문법은 Sized 트레이트에만 적용 가능하다.
    
}
