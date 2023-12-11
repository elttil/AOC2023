// Very ugly and poorly written as I just wanted to be done.
// Don't use this code to try to understand the solution
use std::fs;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn solve(mut v: Vec<(usize, usize, bool)>, empty_x: &Vec<usize>, empty_y: &Vec<usize>, increase: i32) -> u64 {
    let mut pairs = 0;
    let mut sum = 0;
    for outer_index in 0..v.len() {
        v[outer_index].2 = true;
        let o = v[outer_index];
        for inner_index in 0..v.len() {
            let i = v[inner_index];
            if i.2 {
                continue;
            }
            pairs += 1;
            let mut x_diff = (o.0 as i32 - i.0 as i32).abs();
            let x_min = usize::min(o.0, i.0);
            let mut bleh_sum = 0;
            for bleh in x_min..(x_min+x_diff as usize) {
                for e in empty_x {
                    if bleh as usize == *e {
                        bleh_sum += increase-1;
                    }
                }
            }
            x_diff += bleh_sum;
            let mut y_diff = (o.1 as i32 - i.1 as i32).abs();
            let y_min = usize::min(o.1, i.1);
            let mut bleh_sum = 0;
            for bleh in y_min..(y_min+y_diff as usize) {
                for e in empty_y {
                    if bleh as usize == *e {
                        bleh_sum += increase-1;
                    }
                }
            }
            y_diff += bleh_sum;
            let diff = x_diff+y_diff;
            sum += diff as u64;
        }
    }
    sum
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_11/input.txt")?;
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    let mut contains: Vec<Vec<bool>> = vec![];
    for (y, l) in content.lines().enumerate() {
        let mut inn: Vec<bool> = vec![];
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                inn.push(true);
            } else {
                inn.push(false);
            }
        }
        contains.push(inn.clone());
    }

    let mut empty_x: Vec<usize> = vec![];
    let mut empty_y: Vec<usize> = vec![];
    let mut outer_index = 0;
    loop {
        if outer_index >= contains.len() {
            break;
        }
        let mut is_empty = true;
        for i in &contains[outer_index] {
            if *i {
                is_empty = false;
                break;
            }
        }
        if is_empty {
        empty_y.push(outer_index);
        }
        outer_index+=1;
    }
    contains = transpose(contains);
    let mut outer_index = 0;
    loop {
        if outer_index >= contains.len() {
            break;
        }
        let mut is_empty = true;
        for i in &contains[outer_index] {
            if *i {
                is_empty = false;
                break;
            }
        }
        if is_empty {
        empty_x.push(outer_index);
        }
        outer_index+=1;
    }
    contains = transpose(contains);

    let mut v: Vec<(usize, usize, bool)> = vec![];
    for (y, l) in contains.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c {
                v.push((x, y, false));
            }
        }
    }

    println!("part1: {}", solve(v.clone(), &empty_x, &empty_y, 2));
    println!("part2: {}", solve(v.clone(), &empty_x, &empty_y, 1000000));
    Ok(())
}
