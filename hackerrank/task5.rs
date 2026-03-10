fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for d in apples {
        let pos = a + d;
        if pos >= s && pos <= t {
            apple_count += 1;
        }
    }

    for d in oranges {
        let pos = b + d;
        if pos >= s && pos <= t {
            orange_count += 1;
        }
    }

    println!("{}", apple_count);
    println!("{}", orange_count);
}
