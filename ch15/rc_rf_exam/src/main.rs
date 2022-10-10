#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),// b 일때 a가 이동한다. 즉 소유권이 넘어간다.
    Cons(Rc<RefCell<i32>>, Rc<List>), // 소유권을 공유, clone을 호출할 때마다 카운트가 올라감
    Nil,
}
use List::{Cons,Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    *value.borrow_mut()+=10;

    println!("a 수정 후 : {:?}",a);
    println!("b 수정 후 : {:?}",b);
    println!("c 수정 후 : {:?}",c);
}
