fn main() {
    let x = 1;

    // match x {
    //     1 | 2 => println!("1 or 2"),
    //     3 => println!("3"),
    //     _ => println!("그 외의 나머지 값"),
    // }

    match x {
        1 ..= 5 => println!("from 1 to 5 one"),// 5 포함한다.
        // ... 문법은 deprecated. 권장되지 않는 방법론 이며 곧 거부 될 것 21년 기준
        // 문법은 숫자나 char 값만 가능 하다.
        _ => println!("그 외 나머지 값"),
    }
    
    let y = 'v';
    
    match y {
        // ... 문법은 deprecated. 권장되지 않는 방법론 이며 곧 거부 될 것 21년 기준
        // 'a' ... 'j' => println!("ASCII 문자의 전반부"),
        // 'k' ... 'z' => println!("ASCII 문자의 후반부"),
        
        'a' ..= 'j' => println!("ASCII 문자의 전반부"),
        'k' ..= 'z' => println!("ASCII 문자의 후반부"),
        _ => println!("그 외의 나머지 값"),
    }
}
