enum List {
    // Cons(i32, Box<List>),// b 일때 a가 이동한다. 즉 소유권이 넘어간다.
    Cons(i32, Rc<List>), // 소유권을 공유, clone을 호출할 때마다 카운트가 올라감
    Nil,
}
use List::{Cons,Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10,Rc::new(Nil)))));
    println!("a를 생성한 후 카운터 : {}",Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("b를 생성한 후 카운터 : {}",Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c를 생성한 후 카운터 : {}",Rc::strong_count(&a));
    }
    println!("c가 범위를 벗어난 후의 카운터 : {}",Rc::strong_count(&a));
}
