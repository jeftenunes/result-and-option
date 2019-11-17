use std::env::args;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(0, "Hello");
    map.insert(1, "World");

    let result = match map.get(&0) {
        Some(v) => v,
        _ => "Nothing",
    };
 
    println!("{:?}", result);

    println!("{:?}", map.get(&0).unwrap_or(&"No string"));

    match "3".parse::<f32>() {
        Ok(v) => println!("v -> {}", v),
        Err(e) => println!("e -> {}", e),
    }

    match get_arg(3) {
        Ok(v) => println!("v -> {}", v),
        Err(e) => println!("e -> {}", e),
    }
}

fn get_arg(n: usize) -> Result<String, String> {
    for(i, a) in args().enumerate() {
        if i == n {
            return Ok(a);
        }
    }

    return Err("Not enough args".to_string());
}