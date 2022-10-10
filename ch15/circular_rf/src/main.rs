use List::{Cons,Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),// b 일때 a가 이동한다. 즉 소유권이 넘어간다.
    Cons(i32, RefCell<Rc<List>>), // 소유권을 공유, clone을 호출할 때마다 카운트가 올라감
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a의 최초 rc 카운트 : {}",Rc::strong_count(&a));
    println!("a의 다음 아이템 : {:?}",a.tail());
    
    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));
    println!("b를 생성한 뒤 a의 rc 카운트 : {}",Rc::strong_count(&a));
    println!("b의 최초 rc 카운트 : {}",Rc::strong_count(&b));
    println!("b의 다음 아이템 : {:?}",b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    println!("a를 변경한 후 b의 참조 카운트 = {}",Rc::strong_count(&b));
    println!("a를 변경한 후 a의 참조 카운트 = {}",Rc::strong_count(&a));
    
    // println!("a의 다음 아이템 : {:?}",a.tail());
}