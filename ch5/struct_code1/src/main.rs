#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //associated function

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width : 20,
        height : 5
    };

    let rect3 = Rectangle {
        width : 5,
        height : 5,
    };

    let rect4 = Rectangle::square(6);
    println!("Rectangle info : {:#?}",rect1);
    println!("Rectangle area : {}", rect1.area());

    println!("which one is involed?(rect1, rect2) {}",rect1.can_hold(&rect2));
    println!("which one is involed?(rect1, rect3) {}",rect1.can_hold(&rect3));
    println!("rect4 is square, are: {}",rect4.area());

}