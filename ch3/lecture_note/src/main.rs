fn main() {
    //mutablity vs immutablity
    // Default immutablity
    // let x = 5;
    // println!("the vlaue of x: {}", x);
    // x = 6;
    // println!("the vlaue of x : {}",x);
    // But you can assign mutablity for variable
    // let mut x = 5;
    // println!(" the vlaue of x : {}", x);
    // x = 6;
    // println!(" the vlaue of x : {}", x);

    //variable shadowing
    let x = 5;
    let x = x+1;
    let x = x*2;
    println!("{}",x);

    
}
