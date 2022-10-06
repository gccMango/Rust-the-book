use std::collections::HashMap;

use rand::Rng;
use rand::distributions::{Uniform, Standard};
fn main() {
    let mut rng = rand::thread_rng();
    let hundred_range = Uniform::new(0,100);
    let take_range = rng.gen_range(0..100);
    let mut v: Vec<u32> = (&mut rng).sample_iter(hundred_range).take(take_range).collect();

    println!("take range : {}",take_range);
    println!("take range : {}",take_range/2);
    println!("{:?}",v);
    
    println!("{}",average_vec(&v));

    println!("{:?}",v);
    println!("{}",median_vec(&mut v));
    println!("{:?}",v);

    println!("{:?}", count_value(&v));
}

fn average_for(v: &Vec<u32>) {
    // let mut result = 0;
    // for i in &v {
    //     result += i;
    // }
    // println!("{}", result);
    // let sum = result as f32;
}

fn average_vec(v: &Vec<u32>) -> f32 {
    let average = (v.iter().fold(0,|acc,&x| acc+x) as f32)/(v.len() as f32);
    average
}

fn median_vec(v: &mut Vec<u32>) -> f32 {
    v.sort();
    if v.len() % 2 == 1 {
        v[v.len()/2] as f32
    } else {
        ((v[v.len()/2] + v[v.len()/2 -1]) as f32)/2.0
    }
}

fn count_value(v: &Vec<u32>)-> (&u32,u32)  {
    let mut map = HashMap::new();
    let mut count =0;
    for i in v.iter() {
        let count = map.entry(i).or_insert(0);
        *count +=1;
    }
    println!("{:?}",map);

    let result = map.into_iter().max_by_key(|entry| entry.1).unwrap();
    result
}