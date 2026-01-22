fn main() {
    let a = 42;
    let reference = &a;
    let b = a + *reference;

    println!("a + a = {}", b);
}
