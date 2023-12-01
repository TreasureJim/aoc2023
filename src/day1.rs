fn extract_numbers(str: &str) -> u32 {
    let mut numbers = (None, None);

    for char in str.chars() {
        if char.is_ascii_digit() {
            if numbers.0.is_none() {
                numbers.0 = char.to_digit(10);
            } else {
                numbers.1 = char.to_digit(10);
            }
        }
    }

    if numbers.1.is_none() {
        numbers.1 = numbers.0.clone();
    }

    numbers.0.unwrap() * 10 + numbers.1.unwrap() 
}

pub fn calculate_all_numbers(str: &str) -> usize {
    let mut sum: usize = 0;
    for line in str.lines() {
        sum += extract_numbers(line) as usize;
    }
    sum
}

#[cfg(test)]
mod part1 {
    use crate::day1::calculate_all_numbers;

    use super::extract_numbers;

    #[test]
    fn single_line() {
        let test_str = "treb7uchet";
        assert_eq!(extract_numbers(test_str), 77);
    }

    #[test]
    fn whole_str() {
        let test_str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(calculate_all_numbers(test_str), 142);
    }

    #[test]
    fn final_test() {
        let file_str = std::fs::read_to_string("./day1-p1.txt").unwrap();
        dbg!(calculate_all_numbers(&file_str));
    }
}

#[cfg(test)]
mod part2 {
    #[test]
    fn single_line() {
        let test_str = "two1nine";
        todo!()
    }
}
