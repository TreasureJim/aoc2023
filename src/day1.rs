#![allow(unused)]

mod part1 {
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
    mod tests {
        use super::*;

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
        #[ignore]
        fn final_test() {
            let file_str = std::fs::read_to_string("./day1-p1.txt").unwrap();
            dbg!(calculate_all_numbers(&file_str));
        }
    }
}

mod part2 {

    pub fn extract_numbers(s: &str) -> u32 {
        let number_words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut numbers: (Option<u32>, Option<u32>) = (None, None);
        let mut number_indexes = (None, None);

        for (number, word) in number_words.iter().enumerate() {
            let number = number + 1;

            let matches = s.match_indices(word).collect::<Vec<_>>();
            for matching in [matches.first(), matches.last()] {
                let Some((str_index, _)) = matching else {
                    continue;
                };

                if *str_index <= number_indexes.0.unwrap_or(*str_index) {
                    number_indexes.0 = Some(*str_index);
                    numbers.0 = Some(number as u32);
                }
                if *str_index >= number_indexes.1.unwrap_or(*str_index) {
                    number_indexes.1 = Some(*str_index);
                    numbers.1 = Some(number as u32);
                }
            }
        }

        for (char_index, char) in s.chars().enumerate() {
            if char.is_ascii_digit() {
                if char_index <= number_indexes.0.unwrap_or(char_index) {
                    numbers.0 = char.to_digit(10);
                    number_indexes.0 = Some(char_index);
                }
                if char_index >= number_indexes.1.unwrap_or(char_index) {
                    numbers.1 = char.to_digit(10);
                    number_indexes.1 = Some(char_index);
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
            let num = extract_numbers(line) as usize;
            sum += num;
        }
        sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn single_line() {
            let test_str = "7pqrstsixteen";
            assert_eq!(extract_numbers(test_str), 76);
        }

        #[test]
        fn whole_str() {
            let test_str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
            assert_eq!(calculate_all_numbers(test_str), 281);
        }

        #[test]
        #[ignore]
        fn final_test() {
            let file_str = std::fs::read_to_string("./day1-p1.txt").unwrap();
            dbg!(calculate_all_numbers(&file_str));
        }
    }
}
