use std::fs;

fn calculate_usage(s: &str) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    v.resize(36, 0);
    for i in s.chars() {
        if i == 'J' {
            continue;
        }
        let e = {
            if i >= '0' && i <= '9' {
                (i as usize) - '0' as usize + 26
            } else {
                (i as usize) - 'A' as usize
            }
        };
        v[e] += 1;
    }
    v.sort();
    v.reverse();
    for i in s.chars() {
        if i == 'J' {
            v[0] += 1;
        }
    }
    v
}

fn get_rank(s: &str) -> u32 {
    let a: Vec<u32> = calculate_usage(s);
    match a[0] {
        5 => 6,
        4 => 5,
        3 if a[1] == 2 => 4,
        3 => 3,
        2 if a[1] == 2 => 2,
        2 => 1,
        _ => 0,
    }
}

fn char_rank(b: char) -> u32 {
    for (index, c) in "J23456789TQKA".chars().enumerate() {
        if b == c {
            return index as u32;
        }
    }
    0
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_07/input.txt")?;
    let mut part2 = 0;
    let mut vec: Vec<(&str, u32)> = vec![];
    for l in content.lines() {
        let (s, n) = l.split_once(" ").unwrap();
        vec.push((s, n.parse::<u32>().unwrap()));
    }
    vec.sort_by(|(a, _), (b, _)| {
        let tmp_a = get_rank(&a);
        let tmp_b = get_rank(&b);
        if tmp_a > tmp_b {
            return std::cmp::Ordering::Greater;
        }
        if tmp_a < tmp_b {
            return std::cmp::Ordering::Less;
        }
        for (index, _) in a.chars().enumerate() {
            let a_c = char_rank(a.as_bytes()[index] as char);
            let b_c = char_rank(b.as_bytes()[index] as char);
            if a_c > b_c {
                return std::cmp::Ordering::Greater;
            }
            if a_c < b_c {
                return std::cmp::Ordering::Less;
            }
        }
        return std::cmp::Ordering::Equal;
    });
    for (index, (_, w)) in vec.iter().enumerate() {
        part2 += (index + 1) * (*w as usize);
    }
    println!("part2: {}", part2);
    Ok(())
}
