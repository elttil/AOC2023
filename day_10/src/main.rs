// I am ashamed of what I have written.
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    VertNs,
    HorzEw,
    D90Ne,
    D90Nw,
    D90Sw,
    D90Se,
    Ground,
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
struct Location {
    cords: (usize, usize),
    min_cost: u32,
    visited: bool,
    is_outside: bool,
    direction: Direction,
}

#[derive(Clone, Debug, PartialEq)]
struct DotGrouping {
    x_range: Vec<(usize, usize, usize)>,
    count: u32,
    is_loop: bool,
    children: Vec<DotGrouping>,
}

fn get_direction(c: char) -> Direction {
    match c {
        '|' => Direction::VertNs,
        '-' => Direction::HorzEw,
        'L' => Direction::D90Ne,
        'J' => Direction::D90Nw,
        '7' => Direction::D90Sw,
        'F' => Direction::D90Se,
        '.' => Direction::Ground,
        'S' => Direction::Start,
        _ => {
            panic!("Direction not recognized")
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_10/input.txt")?;
    let mut vec: Vec<Vec<Location>> = vec![];
    let mut starting_points: Vec<(usize, usize, Heading, u32)> = vec![];

    let mut start_point: (usize, usize) = (0, 0);
    for (y, l) in content.lines().enumerate() {
        let mut line_vec: Vec<Location> = vec![];
        for (x, c) in l.chars().enumerate() {
            let mut loc: Location = Location {
                cords: (x, y),
                min_cost: 0,
                visited: false,
                is_outside: false,
                direction: get_direction(c),
            };
            if loc.direction == Direction::Start {
                start_point.0 = x;
                start_point.1 = y;
                loc.visited = true;
                if x > 0 {
                    starting_points.push((x - 1, y, Heading::West, 1));
                }
                starting_points.push((x + 1, y, Heading::East, 1));
                starting_points.push((x, y + 1, Heading::South, 1));
                if y > 0 {
                    starting_points.push((x, y - 1, Heading::North, 1));
                }
            }
            line_vec.push(loc);
        }
        vec.push(line_vec.clone());
    }

    let mut complete = false;
    let mut points: Vec<(usize, usize)> = vec![];
    for s in starting_points {
        let mut current = s.clone();
        let mut done = false;

        while !done {
            if current.0 > vec[0].len() {
                println!("OOB");
                break;
            }
            if current.1 > vec.len() {
                println!("OOB");
                break;
            }

            let location: &mut Location = &mut vec[current.1][current.0];
            if location.visited {
                location.min_cost = u32::min(current.3, location.min_cost);
            } else {
                location.min_cost = current.3;
            }
            location.visited = true;

            let old_x = current.0;
            let old_y = current.1;

            current.3 += 1;
            match location.direction {
                Direction::VertNs => {
                    match current.2 {
                        Heading::North => {
                            current.1 -= 1; // north
                        }
                        Heading::South => {
                            current.1 += 1; // south
                        }
                        _ => {
                            done = true;
                        }
                    }
                }
                Direction::HorzEw => {
                    match current.2 {
                        Heading::East => {
                            current.0 += 1; // east
                        }
                        Heading::West => {
                            current.0 -= 1; // west
                        }
                        _ => {
                            done = true;
                        }
                    }
                }
                Direction::D90Ne => {
                    match current.2 {
                        Heading::South => {
                            current.0 += 1; // east
                            current.2 = Heading::East;
                        }
                        Heading::West => {
                            current.1 -= 1; // north
                            current.2 = Heading::North;
                        }
                        _ => {
                            done = true;
                        }
                    }
                }
                Direction::D90Nw => {
                    match current.2 {
                        Heading::South => {
                            current.0 -= 1; // west
                            current.2 = Heading::West;
                        }
                        Heading::East => {
                            current.1 -= 1; // north
                            current.2 = Heading::North;
                        }
                        _ => {
                            done = true;
                        }
                    }
                }
                Direction::D90Sw => {
                    match current.2 {
                        Heading::North => {
                            current.0 -= 1; // west
                            current.2 = Heading::West;
                        }
                        Heading::East => {
                            current.1 += 1; // south
                            current.2 = Heading::South;
                        }
                        _ => {
                            done = true;
                        }
                    }
                }
                Direction::D90Se => {
                    match current.2 {
                        Heading::North => {
                            current.0 += 1; // east
                            current.2 = Heading::East;
                        }
                        Heading::West => {
                            current.1 += 1; // south
                            current.2 = Heading::South;
                        }
                        _ => {
                            done = true;
                        }
                    }
                }
                Direction::Ground => {
                    done = true;
                }
                Direction::Start => {
                    complete = true;
                    done = true;
                }
            };
            if !done && !complete {
                points.push((old_x, old_y));
            }
        }
        if !complete {
            points.clear();
        }
    }
    let mut combination: Vec<Location> = vec![];
    let mut vec_copy = vec.clone();
    for mut v in &mut vec_copy {
        combination.append(&mut v);
    }
    combination = combination
        .into_iter()
        .filter(|x| x.direction != Direction::Ground)
        .collect::<Vec<_>>();
    combination.sort_by_key(|x| x.min_cost);
    combination.reverse();
    println!("part1: {}", combination.first().unwrap().min_cost);

    // Pick's theorem says
    // A = i + (b/2) - 1
    // 'b' is the number of outside points of the loop which is stored in the vector 'points'
    // 'A' is the area of the given polygon(which will be calculated using the shoelace algorithm)
    // 'i' is the number of points inside the loop.
    //
    // Reorganising the equation gives
    // i = A - (b/2) + 1
    points.insert(0, start_point);
    let N = points.len();
    let mut A = 0;
    for i in 0..(N) {
        let mut x1: i32 = points[i].0 as i32;
        let mut y1 = points[i].1 as i32;
        let mut x2: i32;
        let mut y2: i32;
        if i == N-1 {
            x2 = points[0].0 as i32;
            y2 = points[0].1 as i32;
        } else {
            x2 = points[i+1].0 as i32;
            y2 = points[i+1].1 as i32;
        }
        A += (y1+y2)*(x2-x1);
    }
    A = A.abs(); // The area is not negative
    A /= 2; // Shoelace algorithm gives 2*A
    
    let b = N as i32;
    let i = A - (b/2) + 1;

    println!("part2: {}", i);
    Ok(())
}
