fn main() {
    let v = vec!['a','b','c'];

    for (index, value) in v.iter().enumerate() {
        println!("인덱스 {}의 값 : {}", index, value);
    }

}
