// function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}
// fn을 매개변수 타입으로 직접 명시 가능
fn do_twice(f: fn(i32)-> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("Answer is~~~~ {}",answer);

    // let list_of_numbers = vec![1,2,5];
    // let list_of_strings: Vec<String> = list_of_numbers
    // .iter()
    // .map(|i| i.to_string())
    // .collect();
    
    //함수전달
    let list_of_numbers = vec![1,2,5];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop
    }

    let list_of_statuses:Vec<Status> = (0u32..20)
    .map(Status::Value)
    .collect();

    println!("list of status : {:?}",list_of_statuses);
}
