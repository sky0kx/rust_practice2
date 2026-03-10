fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;
        let line = " ".repeat(spaces as usize) + &"#".repeat(hashes as usize);
        println!("{}", line);
    }
}
