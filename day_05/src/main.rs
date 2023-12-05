use std::fs;

#[derive(Clone, Debug)]
struct Mapping {
    dst_range: u64,
    src_range: u64,
    range_length: u64,
}

fn resolve_mapping(input: u64, maps: &Vec<Mapping>) -> u64 {
    for m in maps {
        if input >= m.src_range && input <= (m.src_range + m.range_length) {
            return m.dst_range + (input - m.src_range);
        }
    }
    return input;
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_05/input.txt")?;

    let mut mappings: Vec<Vec<Mapping>> = vec![];

    let mut lines = content.lines();
    let binding = lines
        .nth(0)
        .expect("Expected input")
        .split(":")
        .collect::<Vec<_>>()[1]
        .split(" ")
        .collect::<Vec<_>>();
    let mut inital_seeds = binding.iter().filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<_>>();
    lines.next();

    // This assumes each entry is in the correct order.
    let mut current_map: Vec<Mapping> = vec![];
    for l in lines {
        if l.chars().nth(0).unwrap_or(' ').to_digit(10).is_none() {
                if !current_map.is_empty() {
                    mappings.push(current_map.clone());
                    current_map.clear();
                }
                continue;
        }
        /*
        match l.chars().nth(0).unwrap_or(' ').to_digit(10) {
            Some(_) => {}
            _ => {
                if !current_map.is_empty() {
                    mappings.push(current_map.clone());
                    current_map.clear();
                }
                continue;
            }
        };*/
        let nums = l.split(" ").collect::<Vec<_>>();
        let map: Mapping = Mapping {
            dst_range: nums[0].parse::<u64>().expect(""),
            src_range: nums[1].parse::<u64>().expect(""),
            range_length: nums[2].parse::<u64>().expect(""),
        };
        current_map.push(map);
    }
    // Ugly edge for the last mapping which would not
    // be pushed unless there is a empty line at the
    // end of the input
    if !current_map.is_empty() {
        mappings.push(current_map.clone());
        current_map.clear();
    }

    let mut part1: u32 = std::u32::MAX;
    for s in &inital_seeds {
        let mut point: u32 = *s;
        for map in &mappings {
            point = resolve_mapping(point as u64, map) as u32;
        }
        part1 = u32::min(part1, point);
    }
    println!("part1: {}", part1);

    let mut part2: u64 = std::u64::MAX;
    loop {
        if inital_seeds.is_empty() {
            break;
        }
        let start = inital_seeds[0];
        inital_seeds.remove(0);
        let len = inital_seeds[0];
        inital_seeds.remove(0);
        for s in start..=(start+len) {
            let mut point: u64 = s as u64;
            for map in &mappings {
                point = resolve_mapping(point, map);
            }
            part2 = u64::min(part2, point);
        }
    }

    println!("part2: {}", part2);
    Ok(())
}
