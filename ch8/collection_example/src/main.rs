use std::collections::HashMap;
use std::collections::btree_map::Values;
use std::hash::Hash;
use std::vec;
fn main() {
    //create vec! and Vec<T>
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![];
    let mut v3 = vec![];

    // push the item
    v1.push(5);
    v2.push('a');
    v3.push("a");

    //print
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);

    {
        let v = vec![1, 2, 3, 4];
        //do the anything
    } // escape the scope, v call the drop, will deallocate memory

    // read from vector

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Third elements is {}", third);

    match v.get(2) {
        Some(third) => println!("Third elements is {}", third),
        None => println!("There is not third element"),
    }
    // Below method will be panic.
    // let does_not_exist = &v[100];
    // But This method won't be panic.
    // Because the get method is return None type.
    let does_not_exist = v.get(100);

    // iterator for vector
    for i in &v {
        println!("{}", i);
    }
    // mutable reference for vector
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // vector have only one type
    // But it is little unfortable
    // so we use the enum.
    // using enum, we have one type of vetor but various varients.
    {
        enum SpreedsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreedsheetCell::Int(3),
            SpreedsheetCell::Float(1.23),
            SpreedsheetCell::Text(String::from("Red")),
        ];
    }

    // String &str, string slice

    // let mut s = String::new();

    // let s = "문자열 초깃값".to_string();
    {
        let data = "string init value";

        let s = data.to_string();

        println!("{}", s);
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2: {}", s2);
    }

    {
        let mut s = String::from("lo");
        s.push('l');
        println!("{}", s);
    }
    // append using `+` symbol keyword
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");

        let s3 = s1 + &s2;
        println!("{}", s3);
        println!("{}", s2);
    }
    // dprintln!("{}",s1);// move the ownnership from s1 to s3
    {
        // `+` use the add method
        // fn add(self, s:&str) -> String {}
    }

    //complicated combine the string
    // use the format
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{}", s);
    }

    {
        let hello = "안녕하세요";
        let s = &hello[0..3];
        println!("{}", s);
    }

    {
        for c in "안녕하세요".chars() {
            // seperate the character using char()
            println!("{}", c);
        }

        //also use the bytes() return 1byte
        for b in "안녕하세요".bytes() {
            println!("{}", b);
        }
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"), 10);
        scores.insert(String::from("레드"), 20);

        println!("Hashmap: {:?}", scores);
    }

    {
        let teams = vec![String::from("블루"), String::from("레드")];
        let initial_scores = vec![10,50];

        //zip method return vetor of tuple => collect => HashMap()
        let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

        println!("{:?}", scores);
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("블루");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value at thi point
        // do not valid ownership because move the ownership to map
        println!("map : {:?}", map);
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"),10);
        scores.insert(String::from("Yellow"),50);
        
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        
        println!("{} scored {:?}",team_name, score);
        
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }
    
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"),10);
        scores.insert(String::from("Blue"),25);
        
        println!("{:?}", scores);
    }
    
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"),10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}",scores);

    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count +=1;
        }

        println!("{:?}", map);
    }
}
