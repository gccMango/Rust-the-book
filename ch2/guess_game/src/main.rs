use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's guess a number~");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("user try to target number : {}", secret_number);
    loop {
        println!("input the number if you think it correct.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("input : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("input number is lower than target"),
            Ordering::Greater => println!("input number is bigger than target"),
            Ordering::Equal => {
                println!("collect!!");
                break;
            }
        }
    }
}
