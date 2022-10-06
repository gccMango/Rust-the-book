use std::collections::HashMap;
use std::io;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    println!("Please input string");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("fail to read input");

    let result = pig_latin(input);
    for i in result.iter() {
        print!("{} ", i);
    }
    println!("");
}

fn pig_latin(input: String) -> Vec<String> {
    let mut pig_string: Vec<String> = vec![];

    for item in input.trim().to_lowercase().split_whitespace() {
        pig_string.push(add_suffix(item));
    }

    pig_string
}

fn add_suffix(item: &str) -> String {
    if VOWELS.iter().any(|&v| item.starts_with(v)) {
        let tmp = &item[1..];
        let mut result = String::from(tmp);
        result.push_str("-hay");
        result
    } else {
        let tmp = &item[1..];
        let mut result = String::from(tmp);
        result.push_str("-fay");
        result
    }
}
