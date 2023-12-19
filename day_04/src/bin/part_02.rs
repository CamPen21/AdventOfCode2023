use std::error::Error;
use std::cmp::{max, min};

#[derive(Debug, PartialEq)]
pub struct ScratchCard {
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

pub fn format_line(line: &str) -> Result<ScratchCard, ()> {
    let mut response = ScratchCard {
        winners: vec![],
        numbers: vec![],
    };
    let line: Vec<&str> = line.split(':').collect();
    let line: Vec<&str> = line[1].split('|').collect();
    let winners = line[0].trim();
    let winners: Vec<&str> = winners.split(' ').collect();
    winners.iter().for_each(|number| {
        let num = number.parse::<u32>();
        match num {
            Ok(n) => {
                response.winners.push(n);
            },
            _ => ()
        }
    });
    let numbers = line[1].trim();
    let numbers: Vec<&str> = numbers.split(' ').collect();
    numbers.iter().for_each(|number| {
        let num = number.parse::<u32>();
        match num {
            Ok(n) => {
                response.numbers.push(n);
            },
            _ => ()
        }
    });
    Ok(response)
}

pub fn get_card_score(card: &ScratchCard) -> usize {
    let mut counter = 0;
    for number in card.numbers.iter() {
        if card.winners.contains(&number) {
            counter += 1;
        }
    }
    counter
}

pub fn scratch_cards(input: Vec<ScratchCard>) -> Result<u32, ()> {
    let mut multipliers: Vec<u32> = vec![1; input.len()];
    let mut card_count = 0;
    for (idx, card) in input.iter().enumerate() {
        let card_copies = multipliers[idx];
        let card_score = get_card_score(card);
        println!("{} {}", card_copies, card_score);
        let upper_limit = min(multipliers.len(), 1 + idx + card_score);
        card_count += card_copies;
        for i in idx+1..upper_limit {
            multipliers[i] += card_copies;
        }
        println!("{:?}", multipliers);
    }
    Ok(card_count)
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
    let mut formatted_input = Vec::new();
    let input = include_str!("./day_04_input.txt");
    let input = input.lines();
    for line in input {
        let scratch_card = format_line(line).unwrap();
        formatted_input.push(scratch_card);
    }
    let res = scratch_cards(formatted_input);
    match res {
        Ok(result) => println!("{result}"),
        Err(_) => {
            panic!("error")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parsing() {
        let expected = ScratchCard {
            winners: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 54],
        };
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 54";
        assert_eq!(expected, format_line(input).unwrap());
    }

    #[test]
    fn test_trebuchet() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 54",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let input = input.iter().map(|line| format_line(*line).unwrap()).collect();
        let result = scratch_cards(input).expect("Test failed");
        let expected = 30;
        assert_eq!(
            expected, result,
            "Expected {} received {}",
            expected, result,
        );
    }
}


