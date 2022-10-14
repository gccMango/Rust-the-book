fn main() {
    // let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;
    
    //random access to address
    // let address = 0x012345usize;
    // let r =address as *const size;
    
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}",*r1);
        println!("r2 = {}",*r2);
    }

    
}
