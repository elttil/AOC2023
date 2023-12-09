use std::fs;

fn get_next_line(nums: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut prev: i32 = nums[0];
    for (index, i) in nums.iter().enumerate() {
        if 0 == index {
            continue;
        }
        if nums.len() == index {
            break;
        }
        v.push(*i - prev);
        prev = *i;
    }
    return v;
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_09/input.txt")?;
    let mut part1 = 0;
    let mut part2 = 0;

    for l in content.lines() {
        let mut tree: Vec<Vec<i32>> = vec![];

        let nums: Vec<i32> = l
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().expect(""))
            .collect::<Vec<_>>();
        tree.push(nums.clone());

        let mut previous_vector = nums;

        loop {
            let next = get_next_line(&previous_vector);
            let mut done = true;
            for i in &next {
                if *i != 0 {
                    done = false;
                }
            }
            tree.push(next.clone());
            previous_vector = next.clone();
            if done {
                break;
            }
        }

        let mut to_sum = 0;
        let mut delta = 0;
        for ti in 1..tree.len() {
            let index = tree.len() - 1 - ti;
            let last_value = tree[index].last().unwrap();
            to_sum = last_value + delta;
            delta = last_value + delta;
        }
        part1 += to_sum;

        delta = 0;
        // Backwards extrapolation
        for ti in 1..tree.len() {
            let index = tree.len() - 1 - ti;
            let last_value = tree[index].first().unwrap();
            to_sum = last_value - delta;
            delta = last_value - delta;
        }
        part2 += to_sum;
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
