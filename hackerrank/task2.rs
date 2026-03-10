fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}
