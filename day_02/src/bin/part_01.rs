use std::error::Error;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<GameSet>
}

#[derive(Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32
}

pub fn format_input(raw_input: Vec<&str>) -> Result<Vec<Game>, ()> {
    let mut output = Vec::new();
    for line in raw_input {
        let line_parts: Vec<&str> = line.split(":").collect();
        let game_header = line_parts[0];
        let game_id = game_header.split(" ").collect::<Vec<&str>>()[1].trim();
        let game_id = game_id.parse::<u32>().unwrap();
        let game_sets = line_parts[1];
        let game_sets: Vec<&str> = game_sets.split(";").collect();
        let mut parsed_sets: Vec<GameSet> = Vec::new();
        for set in &game_sets {
            let mut parsed_set = GameSet {
                red: 0,
                green: 0,
                blue: 0
            };
            for draw in set.split(", ") {
                let draw = draw.trim();
                let draw_data = draw.split(" ").collect::<Vec<&str>>();
                let draw_quantity = draw_data[0].parse::<u32>().unwrap();
                let draw_color = draw_data[1].trim();
                match draw_color {
                    "red" => parsed_set.red += draw_quantity,
                    "green" => parsed_set.green += draw_quantity,
                    "blue" => parsed_set.blue += draw_quantity,
                    _ => ()
                }
            }
            parsed_sets.push(parsed_set);

        }
        let game = Game {
            id: game_id,
            sets: parsed_sets,
        };
        output.push(game);
    }
    Ok(output)
}


pub fn games_sum(input: Vec<&str>) -> Result<u32, ()> {
    let mut cubes_left = HashMap::<&str, u32>::new();
    cubes_left.insert("red", 12);
    cubes_left.insert("green", 13);
    cubes_left.insert("blue", 14);
    let mut counter = 0;
    let games = format_input(input).unwrap();
    for game in games {
        let mut is_valid: bool = true;
        for set in &game.sets {
            is_valid &= set.red <= *cubes_left.get("red").unwrap();
            is_valid &= set.blue <= *cubes_left.get("blue").unwrap();
            is_valid &= set.green <= *cubes_left.get("green").unwrap();
        }
        if is_valid {
            counter += game.id
        }
    }
    Ok(counter)
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let input = include_str!("./day_2_input.txt");
    let input = input.lines();
    let res = games_sum(input.collect());
    match res {
        Ok(result) => println!("{result}"),
        Err(_) => {
            panic!("error")
        }
    }
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trebuchet() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let result = games_sum(input).expect("Test failed");
        let expected = 8;
        assert_eq!(
            expected, result,
            "Expected {} received {}",
            expected, result,
        );
    }
}
