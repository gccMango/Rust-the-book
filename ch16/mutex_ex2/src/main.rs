use std::sync::Mutex;
use std::thread;
use std::rc::RC;

fn main() {
    // let counter = Mutex::new(0);
    // try@3
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    // try@1
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num +=1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // try@2
    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();
    //     *num +=1;
    // });
    // handles.push(handle);
    
    // let handle2 = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();
    //     *num +=1;
    // });
    // handles.push(handle2);

    //try@3
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num +=1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    println!("결과 : {}", *counter.lock().unwrap());
}
