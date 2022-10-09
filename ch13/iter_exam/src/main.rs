fn main() {
    let v1 = vec![1,2,3];
    // let v1_iter = v1.iter();

    // for item in v1_iter {
    //     println!("value: {}", item);
    // }

    //iterator adapter
    // 1. map
    let v2 = v1.iter().map(|x| x+1).collect::<Vec<u32>>();
    println!("v1: {:?} v2 : {:?}",v1,v2);

    // 2. filter 
    // check the item and then return boolean type.(true/false)
    // so add the true item in Iterator made by filter
    // But remove the false item in Iterator made by filter
}

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // sum have use the ownership(). so don't use anymore v1_iter
    assert_eq!(total, 6);
}