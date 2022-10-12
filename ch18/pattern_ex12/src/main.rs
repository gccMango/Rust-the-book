struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    // ..(점 두개)를 이용해서 값의 나머지 무시하기
    let orgin = Point { x: 0, y: 0, z: 0 };
    
    match orgin {
        Point {x, ..} => println!("x = {}",x),
    }
    // 튜플에서도 ..(점두개)를 이용해서 그 사이의 값을 무시 할 수 있다.
    let numbers = (2,4,8,16,32);

    match numbers {
        (first, .., last) => {
            println!("first = {}, last = {}",first, last);
        }
    }
}
