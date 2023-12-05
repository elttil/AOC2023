use std::fs;

fn get_digit(l: &str) -> u32 {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (index, n) in nums.iter().enumerate() {
        if l.starts_with(n) {
            return (index + 1) as u32;
        }
    }
    match l.chars().nth(0).unwrap().to_digit(10) {
        Some(n) => n,
        None => 0
    }
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_01/input.txt")?;
    let mut sum = 0;
    for mut l in content.lines() {
        let mut first = 0;
        let mut end = 0;
        while l.len() > 0 {
            let r = get_digit(l);
            l = &l[1..l.len()];
            if 0 == r {
                continue;
            }
            if 0 == first {
                first = r;
            }
            end = r;
        }
        sum += first * 10 + end;
    }
    println!("{}", sum);
    Ok(())
}
