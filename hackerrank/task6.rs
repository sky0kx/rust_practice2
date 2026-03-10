fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if x1 < x2 && v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        "YES"
    } else {
        "NO"
    }
}
