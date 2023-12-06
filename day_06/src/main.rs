use std::fs;

fn get_result(time: i64, distance_to_beat: i64) -> u64 {
    let b = time;
    let c = distance_to_beat;
    // Using quadratic formula(that is taught in Swedish schools).
    // The rounding works for my input but I am unsure if it would work for all inputs.
    let calc = (((b*b/4) - c) as f64).sqrt();
    let lo = b/2 - calc.floor() as i64;
    let hi = (b+1)/2 + calc.ceil() as i64;
    (hi - lo) as u64
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_06/input.txt")?;
    let mut part1 = 1;
    let mut lines = content.lines();
    let times = lines
        .nth(0)
        .expect("Expected input")
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().expect(""))
        .collect::<Vec<_>>();
    let distances = lines
        .nth(0)
        .expect("Expected input")
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().expect(""))
        .collect::<Vec<_>>();
    for (index, t) in times.iter().enumerate() {
        let distance_to_beat = distances[index];
        part1 *= get_result(*t as i64, distance_to_beat as i64);
    }
    println!("part1: {}", part1);

    let time = times.iter().fold(0u64, |sum, x| {
        let log = u64::ilog10(u64::max(*x, 1)) + 1;
        sum * u64::pow(10, log) + x
    });
    let distance = distances.iter().fold(0u64, |sum, x| {
        let log = u64::ilog10(u64::max(*x, 1)) + 1;
        sum * u64::pow(10, log) + x
    });
    let part2 = get_result(time as i64, distance as i64);

    println!("part2: {}", part2);
    Ok(())
}
