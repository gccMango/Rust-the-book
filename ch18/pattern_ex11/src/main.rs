fn main() {
    // variable의 이름을 _로 시작해서 사용하지 않는 변수 무시
    let _x = 5;//그러나 _x는 경고가 나오지 않는다.
    let y = 10;//사용하지 않는 y에 대한 경고 
    println!("Hello, world!");

    //_x는 value를 Binding 하지만, _(그냥 밑줄)은 Binding 하지 않는다.
    // let s = Some(String::from("안녕하세요"));

    // if let Some(_s) = s {
    //     println!("문자열을 찾았습니다.");
    // }
    
    // println!("s는 {:?}", s);// s의 값이 _s로 이동 변수 s 사용 불가
    
    let s = Some(String::from("안녕하세요"));
    // 그냥 _(밑줄)만 사용시 바인디 하지 않는다.
    if let Some(_) = s {
        println!("문자열을 찾았습니다.");
    }

    println!("s는 {:?}", s);
}
