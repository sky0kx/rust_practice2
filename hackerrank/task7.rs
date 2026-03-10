fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    let mut count = 0;

    for x in start..=end {
        if a.iter().all(|&ai| x % ai == 0) &&
           b.iter().all(|&bi| bi % x == 0) {
            count += 1;
        }
    }

    count
}
