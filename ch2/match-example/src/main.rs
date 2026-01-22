fn main() {
    let needle = 42;
    let haystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",

            // !!!Important note!!! If I comment this, Rust warns about the following:
            // error[E0004]: non-exhaustive patterns: `&i32::MIN..=41_i32`, `&43_i32..=131_i32`
            // and `&133_i32..=i32::MAX` not covered
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
