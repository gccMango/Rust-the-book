enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 150, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("ChangeColor: H = {}, S = {}, V = {}", h, s, v);
        }
        _ => {}
    }
}
