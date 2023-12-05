#[derive(Clone)]
struct NumberPoint {
    x: i32,
    y: i32,
    length: i32,
    num: u64,
}

fn main() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("./day_03/input.txt")?;

    let mut number_locations: Vec<NumberPoint> = vec![];

    let mut num: u64 = 0;
    let mut len = 0;
    let mut point: NumberPoint = NumberPoint {
        x: 0,
        y: 0,
        length: 0,
        num: 0,
    };
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    if len == 0 {
                        point.x = x as i32;
                        point.y = y as i32;
                    }
                    num *= 10;
                    len += 1;
                    num += c.to_digit(10).unwrap() as u64;
                }
                _ => {
                    if len > 0 {
                        point.length = len;
                        point.num = num;
                        number_locations.push(point.clone());
                    }
                    num = 0;
                    len = 0;
                }
            }
        }
    }
    // Edge case if no other characthers come after the last number
    if len > 0 {
        point.length = len;
        number_locations.push(point);
    }

    let mut sum = 0;
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }

            let mut ratio = 0;
            let mut n = 0;
            for i in number_locations.iter() {
                if i.y > ((y as i32) + 1) as i32 {
                    break;
                }
                let start = i.x as i32;
                let end = (i.x + i.length) as i32;
                if (x as i32) >= start - 1 && (x as i32) <= end {
                    if ratio == 0 {
                        ratio = 1;
                    }
                    ratio *= i.num;
                    n += 1;
                    if n > 2 {
                        break;
                    }
                }
            }
            if n == 2 {
                sum += ratio;
            }
        }
        number_locations.retain(|e| e.y >= (y as i32));
    }

    println!("{}", sum);
    Ok(())
}
