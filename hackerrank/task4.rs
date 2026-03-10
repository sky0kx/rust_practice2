fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_mult_5 = ((grade / 5) + 1) * 5;
            if next_mult_5 - grade < 3 {
                next_mult_5
            } else {
                grade
            }
        }
    }).collect()
}
