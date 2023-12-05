use std::cmp;
use std::fs;

fn play_game(l: &str) -> i32 {
    let game_state = l.split(":").collect::<Vec<_>>();
    let mut result: [i32; 3] = [0; 3];
    for round in game_state.last().unwrap().split(";") {
        let mut parsed_state: [i32; 3] = [0; 3];
        for cubes in round.split(",") {
            let t = cubes.split(" ").collect::<Vec<_>>();
            let color = *t.last().unwrap();
            let number = t[1].parse::<i32>().unwrap();
            let index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => {
                    panic!("Invalid color")
                }
            };
            parsed_state[index] = number;
        }
        result[0] = cmp::max(result[0], parsed_state[0]);
        result[1] = cmp::max(result[1], parsed_state[1]);
        result[2] = cmp::max(result[2], parsed_state[2]);
    }
    result[0] * result[1] * result[2]
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_02/input.txt")?;
    let mut sum = 0;
    for l in content.lines() {
        sum += play_game(l);
    }
    println!("{}", sum);
    Ok(())
}
