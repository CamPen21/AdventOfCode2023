use std::error::Error;

pub fn trebuchet(input: Vec<&str>) -> Result<u32, ()> {
    let mut global_sum = 0;
    for line in input {
        let mut initial_digit = 0;
        let mut final_digit = initial_digit;
        for char in line.chars() {
            if char.is_digit(10) {
                if initial_digit == 0 {
                    initial_digit = char.to_digit(10).unwrap();
                }
                final_digit = char.to_digit(10).unwrap();
            }
        }
        global_sum += initial_digit*10 + final_digit;
    }
    Ok(global_sum)
}

fn main() -> Result<(), Box<dyn Error + 'static>>{
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
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let result = trebuchet(input).expect("Test failed");
        let expected = 142;
        assert_eq!(
            expected, result,
            "Expected {} received {}",
            expected, result,
        );
    }
}
