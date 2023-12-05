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
    let mut part1 = 0;
    let mut part2 = 0;
    for mut l in content.lines() {
        let mut first = 0;
        let mut end = 0;
        for c in l.chars() {
            if !c.is_digit(10) {
                continue;
            }
            let digit = c.to_digit(10).expect("");
            if first == 0 {
                first = digit;
            }
            end = digit;
        }
        part1 += first * 10 + end;
        first = 0;
        end = 0;
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
        part2 += first * 10 + end;
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
