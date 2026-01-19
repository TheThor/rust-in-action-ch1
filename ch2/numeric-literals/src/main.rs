fn main() {
    let twenty = 20;
    let twenty_one = 21i32;
    let twenty_two = 22_i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million = 1_000_000_i32;
    println!("{}", one_million.pow(2));

    let test : f32 = one_million as f32;
    let forty_twos = [
        42.0,
        42f32,
        42_f32,
        test,
        one_million as f32,
    ];

    println!("{:02}", forty_twos[0]);
}
