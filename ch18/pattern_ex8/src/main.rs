enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn main() {
    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Quit;
    // let msg = Message::Move { x: 0, y: 10 };
    let msg = Message::Write("hello message".to_string());

    match msg {
        Message::Quit => {
            println!("Quit: 해체할 값이 없습니다.")
        }
        Message::Move { x, y } => {
            println!("Move: x = {}, y = {}", x, y)
        }
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b)
        }
    }
}
