use std::{collections::HashMap, error::Error};


fn get_line_digit(line: &str) -> u32 {
    let mut found_digits: Vec<u32> = vec![];
    let digits: Vec<(&str, u32)> = vec![
        ("one", 1), 
        ("two", 2), 
        ("three", 3), 
        ("four", 4), 
        ("five", 5), 
        ("six", 6), 
        ("seven", 7), 
        ("eight", 8), 
        ("nine", 9) 
    ];
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    for idx in 0..n {
        let first_char = chars[idx];
        if first_char.is_digit(10) {
            found_digits.push(first_char.to_digit(10).unwrap());
            continue
        }
        for digit in &digits {
            if line[idx..].starts_with(digit.0) {
                found_digits.push(digit.1); }
        };
    }
    let mut line_number: u32 = 0;
    match found_digits.get(0) {
        Some(value) => line_number = value*10,
        None => (),
    }
    match found_digits.pop() {
        Some(value) => line_number += value,
        None => (),
    }
    line_number
}

pub fn trebuchet(input: Vec<&str>) -> Result<u32, ()> {
    let mut convert = HashMap::<&'static str, u32>::new();
    convert.insert("one", 1);
    convert.insert("two", 2);
    convert.insert("three", 3);
    convert.insert("four", 4);
    convert.insert("five", 5);
    convert.insert("six", 6);
    convert.insert("seven", 7);
    convert.insert("eight", 8);
    convert.insert("nine", 9);
    let mut global_sum = 0;
    for line in input {
        let line_digit = get_line_digit(line);
        global_sum += line_digit;
    }
    Ok(global_sum)
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let input = include_str!("./day_1_input.txt");
    let input = input.lines();
    let res = trebuchet(input.collect());
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
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let result = trebuchet(input).expect("Test failed");
        let expected = 281;
        assert_eq!(
            expected, result,
            "Expected {} received {}",
            expected, result,
        );
    }
}
