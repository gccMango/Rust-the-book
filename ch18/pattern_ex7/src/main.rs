struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p  = Point {x:0, y:5};
    
    // let Point {x: a, y: b} = p; // 구조체 해체
    // assert_eq!(0,a);
    // assert_eq!(5,b);

    // 구조체 필드 명을 동일하게 해서 통일감을 줄 수 있다.
    // let Point {x,y} = p;

    // assert_eq!(0,x);
    // assert_eq!(5,y);

    match p {
        Point {x, y:0} => println!("x축 {}에 위치하는 점", x),
        Point {x:0, y} => println!("y축 {}에 위치하는 점",y),
        Point {x,y} => println!("좌표 ({}, {})에 위치하는 점", x,y),
    }
}
