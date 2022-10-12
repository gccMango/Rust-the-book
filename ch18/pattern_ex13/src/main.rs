fn main() {
    // match 패턴을 가지고 충분하지 않을 때가 있다.
    // 추가적으로 if 조건을 지정해서 그 조건이 일치 할 때만 그 가지를 실행 할 수 있다.
    // 복잡한 조건일 떄 유용하다.
    // match guard
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("5보다 작은 값 : {}", x),
        Some(x) => println!("x:{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x{
        Some(50) => println!("50"),
        Some(n) if n ==y => println!("일치, n= {:?}",n),
        _ => println!("일치하지 않음, x = {:?}",x),
    }

    println!("결과: x = {:?}, y = {:?}",x,y);

    {
        let x= 4;
        let y = false;
        
        match x {
            4 | 5| 6 if y => println!("yes~!!"),
            _ => println!("No~!~!"),
        }
    }
}
