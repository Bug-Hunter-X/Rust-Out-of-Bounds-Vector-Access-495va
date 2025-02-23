fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let val = vec[2]; //this line will panic at runtime
    println!("The value is {}", val);
}