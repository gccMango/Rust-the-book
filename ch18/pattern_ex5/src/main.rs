fn main() {
    // let x = 1;

    // match x {
    //     1 => println!("하나"),
    //     2 => println!("둘"),
    //     3 => println!("셋"),
    //     _ => println!("나머지"),
    // }

    // shadow variable 
    // match expression make new area, so y is made by new area
    // new y shadow the old one.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(y) =>println!("일치, y = {:?}",y),
        _ => println!("일치하지 않음, x= {:?}", x),
    }

    println!("결과 x: {:?} y: {:?}", x, y);
}
