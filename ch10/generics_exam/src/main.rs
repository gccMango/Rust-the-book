// use core::num;

fn main() {
    let number_list = vec![20,30,50,50,20,80];

    let largest_num = largest(&number_list);
    println!("The largest number is {}", largest_num);
}

fn largest(number_list: &Vec<i32>) -> i32 {
    let mut largest_num = number_list[0];
    for &item in number_list {

        if item > largest_num {
            largest_num = item;
        }
    }
    largest_num
}
