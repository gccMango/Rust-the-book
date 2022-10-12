enum Message {
    Hello { id: i32},
}
fn main() {

    //@ at 연산자는 어떤값이 패턴과 일치하는지 비교하는 동시에
    // 그 값을 가진 변수를 생성한다.
    let msg = Message::Hello {id: 5};
    match msg {
        Message::Hello {id: id_variable @ 3..=7} =>{
            println!("id를 범위에서 찾았습니다.: {}",id_variable)
        },
        Message::Hello {id: 10..=12} =>{
            println!("id를 다른 범위에서 찾았습니다.")
            // 여기서는 id 값을 변수에 저장하지 않아서 id 필드값을 이용 불가
        },
        Message::Hello {id} =>{
            println!("다른 id를 {} 찾았습니다.",id)
            //범위없이 선언했으므로 변수 그 값 자체로 사용가는 하다.
            // id 값을 비교 하지 않고, 이 패턴은 어떤 값이든 일치한다.
        },
    }
}
