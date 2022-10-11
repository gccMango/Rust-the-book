use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("안녕하세요.");
        tx.send(val).unwrap();
        // println!("val = {}", val); //소유권 이동으로 수신자에게 넘어 갔다. 
    });


    let recevied = rx.recv().unwrap();
    println!("수신: {}",recevied);
}
