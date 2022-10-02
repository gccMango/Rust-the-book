use std::io;
fn main() {
    println!("Let's guess a number~");
    println!("input the number if you think it correct.");

    let mut guess =String::new();

    io::stdin().read_line(&mut guess)
               .expect("failed to read the input.");

    println!("input : {}", guess);
}
