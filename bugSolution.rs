fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access elements: check bounds first
    if vec.len() > 10 {
        let value = vec[10];
        println!("Value at index 10: {}", value);
    } else {
        println!("Index 10 is out of bounds");
    }

    // Or use get() for safe access, which returns an Option
    if let Some(value) = vec.get(10) {
        println!("Value at index 10: {}", value);
    } else {
        println!("Index 10 is out of bounds");
    }
} 