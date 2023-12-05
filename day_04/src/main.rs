use std::fs;

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("./day_04/input.txt")?;

    let mut vec: Vec<usize> = vec![];
    let mut part1 = 0;
    let mut part2 = 0;
    for (index, l) in content.lines().enumerate() {
        let (_, card_state) = l.split_once(":").unwrap();
        let (win, card) = card_state.split_once("|").unwrap();
        let winning_numbers = win
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let card_numbers = card
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let my_winning_numbers: Vec<_> = card_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .collect();

        let found = my_winning_numbers.len();
        
        if found != 0 {
            part1 += u32::pow(2, found as u32-1);
        }

        let value = vec.get(index).unwrap_or(&0) + 1;
        part2 += value;
        vec.resize(vec.len() + found + 1, 0);
        for i in 1..=found {
            vec[index + i] += value;
        }
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
