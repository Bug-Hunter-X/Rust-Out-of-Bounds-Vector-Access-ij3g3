fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will cause a runtime error!
    let value = vec[10];
    println!("Value at index 10: {}", value);
}