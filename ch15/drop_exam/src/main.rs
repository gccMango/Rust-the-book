struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: "내데이터".to_string(),
    };
    let b = CustomSmartPointer {
        data: "다른 데이터".to_string(),
    };

    println!("CustomSmartPointer를 생성했습니다.");

    drop(c);// std::mem::drop vs Drop 트레이트의 drop과 다르다.

    println!("CustomSmartPointer를 main함수의 끝에 도달하기 전에 해제합니다.");

}
