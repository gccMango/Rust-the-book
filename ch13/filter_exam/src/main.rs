#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_mysize(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneakers".to_string(),
        },
        Shoe {
            size: 13,
            style: "sandal".to_string(),
        },
        Shoe {
            size: 10,
            style: "boots".to_string(),
        },
    ];

    let in_my_size = shoes_in_mysize(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: "sneakers".to_string()
            },
            Shoe {
                size: 10,
                style: "boots".to_string()
            },
        ]
    )
}
fn main() {
    println!("Hello, world!");
}
