use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;

fn main() {
    // use match epression
    // let f= File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     // Err(error) => panic!("fail to open file : {:?}",error),
    //     Err(ref error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("파일을 생성하지 못했습니다.: {:}",e),
    //         },
    //         other_error => panic!("파일을 열지 못했습니다.: {:}", other_error),
    //     }
    // };
    
}

fn read_username_from_file() -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
    
}

fn read_username_from_file_v2() -> Result<String,io::Error> {
    
    let mut s = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
    
}

fn read_username_from_file_v3() -> Result<String,io::Error> {
    fs::read_to_string("hello.txt")?;
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }