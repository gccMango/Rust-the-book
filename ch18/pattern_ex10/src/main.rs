fn main() {
    foo(3,4);

    // _를 중첩해서 값의 일부를 무시하기
    // let mut setting_value = Some(5);
    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_),Some(_)) => {
            println!("이미 설정된 값을 덮어 쓸 수 없습니다.");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("현재 설정 : {:?}",setting_value);

    //tuple에서 일부 값 무시하는 _ 패턴
    let numbers = (2,4,8,16,32);

    match numbers {
        (first,_, third,_, fifth) => {
            println!("일치하는 숫자: {}, {}, {}", first, third, fifth)
        }
    }
}

fn foo(_: i32, y: i32) {
    // _ 패턴은 값 전체를 무시 할 수 있다.
    println!("이 함수는 y 매개변수만 사용한다: {}", y);
}
