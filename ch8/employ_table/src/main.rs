use std::collections::HashMap;
use std::io;
fn main() {
    let mut map = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Please the first name and position");
        println!("format : add {{your last-name}} to {{Department}}");
        io::stdin()
            .read_line(&mut input)
            .expect("fail to read the input");
        let input_string = input.trim().to_lowercase();
        let mut vec: Vec<&str> = vec![];

        for item in input_string.split_whitespace() {
            if item == "add" || item == "to" {
                continue;
            } else {
                vec.push(item);
            }
        }
        map.insert(vec[0].to_string(), vec[1].to_string());
        println!("{:?}", map);

        let mut answer = String::new();

        println!(" Do you want to arrange the map in alphabetic order? [y/n]");
        io::stdin()
            .read_line(&mut answer)
            .expect("fail to read the answer");
        answer.clear();
        // fun_name();
        println!("Do you register more? [y/n]");
        io::stdin()
            .read_line(&mut answer)
            .expect("fail to read the answer");
        if answer.trim() == "n" {
            break;
        }
    }
}

fn fun_name() {
    // if answer.trim() == "y" {
    //     answer.clear();
    //     println!("if you want to order all employee, input : all");
    //     println!(
    //         "Or want to order by Departmet,
    //     input : {{Department}}"
    //     );
    //     io::stdin()
    //         .read_line(&mut answer)
    //         .expect("fail to read the input");
    //     let input_answer = answer.trim().to_lowercase();
    //     if input_answer == "all" {
    //         let mut result: Vec<_> = map.into_keys().collect();
    //         result.sort();
    //         println!("{:?}", result);
    //     } else {
    //     }
    // } else {
    //     answer.clear();
    // }
}
