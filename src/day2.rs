#![allow(unused)]

fn parse_input(s: &str) -> Vec<[usize; 3]> {
    let s = &s[s.find(": ").unwrap() + 2..];
    let s: Vec<_> = s
        .split("; ")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.split(", ").collect::<Vec<_>>())
        .collect();

    const COLORS: [&'static str; 3] = ["red", "green", "blue"];

    let mut rounds = Vec::new();

    for round in s {
        let mut num_cubes = [0, 0, 0];
        for pick in round {
            for (c_i, color) in COLORS.iter().enumerate() {
                if pick.contains(color) {
                    num_cubes[c_i] = pick.split_whitespace().next().unwrap().parse().unwrap();
                    break;
                }
            }
        }

        rounds.push(num_cubes);
    }

    rounds
}

fn is_possible(input: &[[usize; 3]], red: usize, green: usize, blue: usize) -> bool {
    for round in input {
        if round[0] > red || round[1] > green || round[2] > blue {
            return false;
        }
    }

    true
}

mod part1 {
    use super::{parse_input, is_possible};

    fn find_possible(s: &str, red: usize, green: usize, blue: usize) -> usize {
        let mut sum = 0;

        for (line_num, line) in s.lines().enumerate() {
            if is_possible(&parse_input(line), red, green, blue) {
                println!("{}", line);
                sum += line_num + 1;
            }
        }

        sum
    }

    #[cfg(test)]
    mod test {
        use crate::day2::part1::{find_possible, is_possible, parse_input};

        #[test]
        fn test_parse() {
            let test_str =
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
            assert_eq!(parse_input(test_str), [[3, 1, 6], [6, 3, 0], [14, 3, 15]]);
        }

        #[test]
        fn test_is_possible() {
            let test_str =
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
            assert_eq!(is_possible(&parse_input(test_str), 12, 13, 14), false);
        }

        #[test]
        fn test_parse2() {
            let test_str = "Game 4: 1 red, 3 blue; 3 blue, 3 green, 1 red; 11 blue, 2 green; 2 green, 14 blue; 1 green, 7 blue; 11 blue, 5 green";
            assert_eq!(
                parse_input(test_str),
                [
                    [1, 0, 3],
                    [1, 3, 3],
                    [0, 2, 11],
                    [0, 2, 14],
                    [0, 1, 7],
                    [0, 5, 11]
                ]
            );
        }

        #[test]
        fn test_is_possible2() {
            let test_str = "Game 4: 1 red, 3 blue; 3 blue, 3 green, 1 red; 11 blue, 2 green; 2 green, 14 blue; 1 green, 7 blue; 11 blue, 5 green";
            assert_eq!(is_possible(&parse_input(test_str), 12, 13, 14), true);
        }

        #[test]
        fn whole() {
            let test_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
            assert_eq!(find_possible(test_str, 12, 13, 14), 8);
        }

        #[test]
        fn test_final() {
            let file = std::fs::read_to_string("day2.txt").unwrap();
            dbg!(find_possible(&file, 12, 13, 14));
        }
    }
}

mod part2 {
    use super::parse_input;

    fn find_min(input: &[[usize; 3]]) -> (usize, usize, usize) {
        input
            .iter()
            .fold((0, 0, 0), |acc, [x, y, z]| {
                (acc.0.max(*x), acc.1.max(*y), acc.2.max(*z))
            })
    }

    fn sum_powers(s: &str) -> usize {
        s.lines().map(|x| find_min(&parse_input(x))).fold(0, |acc, (x, y, z)| acc + x * y * z)
    }

    mod tests {
        use crate::day2::part2::sum_powers;

        #[test]
        fn sum_powers_test() {
            let test_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
            assert_eq!(sum_powers(test_str), 2286);
        }

        #[test]
        fn final_test() {
            let file = std::fs::read_to_string("day2.txt").unwrap();
            dbg!(sum_powers(&file));
        }
    }
}
