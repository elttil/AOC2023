use std::collections::HashMap;
use std::fs;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }

    if b > a {
        let temp = b;
        b = a;
        a = temp;
    }

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_08/input.txt")?;
    let mut lines = content.lines();
    let moves = lines.nth(0).unwrap();
    lines.nth(0);

    let mut locations: HashMap<&str, (String, String)> = HashMap::new();
    let mut a_nodes: Vec<&str> = vec![];
    for l in lines {
        let loc = l.split_once(" = ").unwrap();
        let binding = loc.1.replace(&['(', ')', ','][..], "");
        let rest = binding.split_once(" ").unwrap();
        locations.insert(loc.0, (rest.0.to_string(), rest.1.to_string()));
        if loc.0.chars().last().unwrap() == 'A' {
            a_nodes.push(loc.0);
        }
    }

    let mut loc_string = "AAA";
    let mut location = locations.get(loc_string).unwrap();
    let mut part1 = 0;
    loop {
        if loc_string == "ZZZ" {
            break;
        }
        for m in moves.chars() {
            if m == 'L' {
                loc_string = &location.0;
            } else if m == 'R' {
                loc_string = &location.1;
            }
            location = locations.get(loc_string).unwrap();
            part1 += 1;
        }
    }
    println!("part1: {}", part1);

    let mut multi: Vec<u32> = vec![];
    for a in a_nodes {
        let mut loc_string = a;
        let mut depth = 0;
        loop {
            for m in moves.chars() {
                if loc_string.chars().last().unwrap() == 'Z' {
                    break;
                }
                let location = locations.get(loc_string).unwrap();
                if m == 'L' {
                    loc_string = &location.0 as &str;
                } else if m == 'R' {
                    loc_string = &location.1 as &str;
                }
                depth += 1;
            }
            if loc_string.chars().last().unwrap() == 'Z' {
                break;
            }
        }
        multi.push(depth);
    }

    let mut prev = 1;
    for m in &multi {
        prev = lcm(prev, *m as u64);
    }
    println!("part2: {}", prev);
    /*
    Old slower solution. Apparently this is just "lcm" which is used above.
    multi.sort();
    let min: u64 = multi[0] as u64;
    let mut result: u64 = min;
    let mut num = 1;
    loop {
        let mut done = true;
        for m in &multi {
            if result % *m as u64 != 0 {
                done = false;
                break;
            }
        }
        if done == true {
            break;
        }
        num += 1;
        result = min*num;
    }
    println!("part2: {}", num*min);
    */
    Ok(())
}
