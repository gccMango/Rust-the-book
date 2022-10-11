use std::thread;

fn main() {
    let v = vec![1,2,3];
    
    let handle = thread::spawn(move ||{ //클로저가 필요로 하는 값을 대여할려는 동작을 고치고 소유권을 갖게 한다.
        println!("벡터 : {:?}",v);
    });
    // drop(v);
    // 주 스레드가 drop 함수를 이용해서 곧 바로 v를 해제해 버린다.
    // 자식 스레드가 실행되는 시점에서는 v는 더 이상 유효하지 않다.
    handle.join().unwrap();
}
