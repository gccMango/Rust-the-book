use std::fmt;
//new type pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"[ {} ]",self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("안녕하세요~"), String::from("러스트입니다.")]);

    println!("w = {}", w);
}
