// use std::cmp::PartialOrd;
// fn main() {

// }

// fn largest<T>(list: &Vec<T>) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

struct Pointv2<T,U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Pointv2<i32,i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}
impl<T,U> Pointv2<T,U> {
    fn mixup<V,W>(self, other: Pointv2<V,W>) -> Pointv2<T,W> {
        Pointv2 { x: self.x, y: other.y }
    }
}
fn main() {
    let point1 = Point {
        x: 1.0,
        y: 2.0,
    };
    let point2 = Pointv2 {
        x: 2,
        y: 2.0
    };
    let point3 = Pointv2 { x: "hello", y: 'y' }; 

    let point4 = point2.mixup(point3);

    println!("Point x : {}",point1.x());
    println!("Point x : {} y: {}",point4.x,point4.y);
}