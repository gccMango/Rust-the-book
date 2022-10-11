use std::vec;

use trait_ex1::Draw;
use trait_ex1::{Screen, Button};

struct SelectionBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectionBox {
    fn draw(&self) {
        // selecbox를 그리는 코드
    }
}

fn main() {
    let screen= Screen{
        components: vec![
            Box::new(SelectionBox{
                width: 75,
                height: 10,
                options: vec![
                    "예".to_string(),
                    "아니요".to_string(),
                    "모름".to_string(),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "확인".to_string(),
            }),
        ],
    };
    // Draw 트레이트를 구현하지 않으면 String 타입은 컴파일 에러가 발생
    screen.run();
}