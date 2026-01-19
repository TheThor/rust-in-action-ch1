fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 40_i32;

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);

    let e = add(add(a, b), add(c, d));

    println!("sum all: {:?}", e);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}