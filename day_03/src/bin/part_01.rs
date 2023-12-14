use std::error::Error;

#[derive(Debug)]
struct Direction {
    r: isize,
    c: isize,
}

const DIRECTIONS: [Direction; 8] = [
    Direction{ r: -1, c: 0},
    Direction{ r: -1, c: 1},
    Direction{ r: 0, c: 1},
    Direction{ r: 1, c: 1},
    Direction{ r: 1, c: 0},
    Direction{ r: 1, c: -1},
    Direction{ r: 0, c: -1},
    Direction{ r: -1, c: -1},
];

pub fn format_input(raw_input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in raw_input.lines() {
        matrix.push(line.chars().collect());
    };
    matrix
}

fn is_dot(character: &char) -> bool {
    *character == '.'
}

fn is_digit(character: &char) -> bool {
    character.is_digit(10)
}

fn get(matrix: &Vec<Vec<char>>, y: usize, x: usize, dy: isize, dx: isize) -> Option<&char> {
    let row = dy + y as isize;
    let col = dx + x as isize;
    if row < 0 || col < 0 {
        return None;
    }
    let row = matrix.get(row as usize)?;
    row.get(col as usize)
}


pub fn is_valid_position(matrix: &Vec<Vec<char>>, row: isize, col: isize) -> bool {
    row >= 0 && col >= 0 && row < matrix.len() as isize && col < matrix[0].len() as isize
}

pub fn find_number_in_line(line: &Vec<char>, pos: usize) -> u32 {
    let mut left = pos;
    let mut right = pos;
    while left >= 1 {
        if !is_digit(&line[left-1]) {
            break;
        }
        left -= 1;
    }
    while right < line.len()-1 {
        if !is_digit(&line[right+1]) {
            break;
        }
        right += 1;
    }
    let number = String::from_iter(&line[left..=right]);
    let number = number.parse::<u32>();
    number.unwrap_or(0)
}

pub fn gear_ratios(input: Vec<Vec<char>>) -> Result<u32, ()> {
    let mut numbers = Vec::<u32>::new();
    for y in 0..input.len() {
        let mut buffer = String::new();
        let mut check = true;
        for x in 0..input[0].len() {
            let curr = get(&input, y, x, 0, 0).unwrap();
            let is_number = is_digit(curr);

            if is_number {
                buffer.push(*curr);
            }

            if !is_number && !check{
                if buffer.len() > 3 {
                    println!("{} => {:?}", buffer, input[y]);
                }
                let res = buffer.parse::<u32>();
                if let Ok(number) = res {
                    numbers.push(number);
                }
            }
            if !is_number {
                buffer.clear();
                check = true;
                continue;
            }
            if is_number && check {
                let is_part_number: bool = DIRECTIONS.iter().fold(false, |acc, dir| {
                    let _char = get(&input, y, x, dir.r, dir.c);
                    match _char {
                        Some(found) => {
                            acc || !is_digit(found) && !is_dot(found)
                            },
                        None => acc
                    }
                });
                if is_part_number {
                    check = false;
                }
            }
        }
        if !check {
            let res = buffer.parse::<u32>();
            if let Ok(number) = res {
                numbers.push(number);
            }
        }
    }
    println!("{:?}", numbers);
    Ok(numbers.iter().sum())
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let input = include_str!("./day_03_input.txt");
    let matrix = format_input(input);
    let res = gear_ratios(matrix);
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
    fn test_trebuchet() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let input = format_input(input.join("\n").as_str());
        let result = gear_ratios(input).expect("Test failed");
        let expected = 4361;
        assert_eq!(
            expected, result,
            "Expected {} received {}",
            expected, result,
        );
    }

    #[test]
    fn test_is_digit() {
        let input = '.';
        assert_eq!(false, is_digit(&input));
        let input = '1';
        assert_eq!(true, is_digit(&input));
    }
}
