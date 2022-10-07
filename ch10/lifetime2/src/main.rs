use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("모두 주목해주세요~ {}", announcement);
        self.part
    }
}
fn longest_with_an_announcement<'a,T> (x: &'a str, y:&'a str, ann: T) -> &'a str 
where T: Display {
    println!("주목하세요~{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let novel =String::from("원피스. 대항해 시대 해적들이 날뛴다.");
    let first_sentence = novel.split('.').next().expect("문장에서 '.'을 찾을 수 없습니다.");
    let i  = ImportantExcerpt{part: first_sentence};
    println!("{}",i.part);
    i.announce_and_return_part("원~~~피스");
    longest_with_an_announcement(&novel, "bcd", "시작합니다!");
}
