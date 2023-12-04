#![allow(unused)]

use std::ops::Range;

use regex::Regex;

fn find_numbers(s: &str) -> Vec<(usize, Range<usize>)> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(s)
        .map(|x| (x.as_str().parse().unwrap(), x.range()))
        .collect()
}

mod part1 {
    use super::*;

    fn find_symbols(s: &str) -> Vec<usize> {
        Regex::new(r"[^\d\s.]")
            .unwrap()
            .find_iter(s)
            .map(|x| x.start())
            .collect()
    }

    fn next_to_symbol(
        range: &Range<usize>,
        prev: Option<&[usize]>,
        curr: Option<&[usize]>,
        next: Option<&[usize]>,
    ) -> bool {
        let range = (range.start.saturating_sub(1))..=(range.end);
        for sym in prev.unwrap_or_default() {
            if range.contains(sym) {
                return true;
            }
        }
        for sym in curr.unwrap_or_default() {
            if range.contains(sym) {
                return true;
            }
        }
        for sym in next.unwrap_or_default() {
            if range.contains(sym) {
                return true;
            }
        }

        false
    }

    fn sum_valid_parts(s: &str) -> usize {
        let mut previous_symbols: Option<Vec<usize>> = None;

        let mut next_line_iter = s.lines();
        let mut current_symbols = Some(find_symbols(next_line_iter.next().unwrap()));
        let mut next_symbols = Some(find_symbols(next_line_iter.next().unwrap()));

        let mut sum = 0;

        for line in s.lines() {
            sum += find_numbers(line)
                .into_iter()
                .filter(|x| {
                    next_to_symbol(
                        &x.1,
                        previous_symbols.as_deref(),
                        current_symbols.as_deref(),
                        next_symbols.as_deref(),
                    )
                })
                .map(|x| x.0)
                .sum::<usize>();

            previous_symbols = current_symbols.take();
            current_symbols = next_symbols.take();
            if let Some(next_line) = next_line_iter.next() {
                next_symbols = Some(find_symbols(next_line));
            }
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sum() {
            let test_str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
            assert_eq!(sum_valid_parts(test_str), 4361);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day3.txt").unwrap();
            dbg!(sum_valid_parts(&test_file));
        }
    }
}

mod part2 {
    use std::ops::Mul;

    use super::*;

    fn is_gear(
        loc: usize,
        prev: Option<&[(usize, Range<usize>)]>,
        curr: Option<&[(usize, Range<usize>)]>,
        next: Option<&[(usize, Range<usize>)]>,
    ) -> Option<usize> {
        let mut nums = Vec::with_capacity(2);

        for (value, range) in prev.unwrap_or_default() {
            let range = (range.start.saturating_sub(1))..=(range.end);
            if range.contains(&loc) {
                nums.push(value);
                if nums.len() > 2 {
                    return None;
                }
            }
        }

        for (value, range) in curr.unwrap_or_default() {
            let range = (range.start.saturating_sub(1))..=(range.end);
            if range.contains(&loc) {
                nums.push(value);
                if nums.len() > 2 {
                    return None;
                }
            }
        }

        for (value, range) in next.unwrap_or_default() {
            let range = (range.start.saturating_sub(1))..=(range.end);
            if range.contains(&loc) {
                nums.push(value);
                if nums.len() > 2 {
                    return None;
                }
            }
        }

        if nums.len() != 2 {
            return None;
        }

        Some(nums.into_iter().fold(1, |acc, x| acc.mul(x)))
    }

    fn sum_valid_gears(s: &str) -> usize {
        let mut previous_nums = None;

        let mut next_line_iter = s.lines();
        let mut current_nums = Some(find_numbers(next_line_iter.next().unwrap()));
        let mut next_nums = Some(find_numbers(next_line_iter.next().unwrap()));

        let mut sum = 0;

        for line in s.lines() {
            sum += line
                .match_indices('*')
                .filter_map(|x| {
                    is_gear(
                        x.0,
                        previous_nums.as_deref(),
                        current_nums.as_deref(),
                        next_nums.as_deref(),
                    )
                })
                .sum::<usize>();

            previous_nums = current_nums.take();
            current_nums = next_nums.take();
            if let Some(next_line) = next_line_iter.next() {
                next_nums = Some(find_numbers(next_line));
            }
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sum() {
            let test_str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
            assert_eq!(sum_valid_gears(test_str), 467835);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day3.txt").unwrap();
            dbg!(sum_valid_gears(&test_file));
        }
    }
}
