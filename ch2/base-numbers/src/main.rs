fn main() {
    let three = 0b11; // Mean binary 11 is equal to 3
    let _one = 0b01; // Mean binary 01 is equal to 1
    let thirty = 0o36; // Octo base representation of 30
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
