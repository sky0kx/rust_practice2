fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut count = [0; 5];

    for &id in arr {
        count[(id - 1) as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 0..5 {
        if count[i] > max_count {
            max_count = count[i];
            result = i + 1;
        }
    }

    result as i32
}
