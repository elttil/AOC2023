use std::fs;

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
            let mut v: Vec<i32> = vec![];
            for i in previous_vector.windows(2) {
                v.push(i[1] - i[0]);
            }
            tree.push(v.clone());
            previous_vector = v;
            if previous_vector.iter().all(|&x| x == 0) {
                break;
            }
        }

        part1 += tree.iter().fold(0i32, |mut delta, x| {
            delta = x.last().unwrap() + delta;
            delta
        });
        part2 += tree.iter().rev().fold(0i32, |mut delta, x| {
            delta = x.first().unwrap() - delta;
            delta
        });
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
