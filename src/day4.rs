#![allow(unused)]

use std::collections::HashSet;

fn parse_input(s: &str) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut results = Vec::new();

    for line in s.lines() {
        let line = &line[line.find(": ").unwrap() + 2..];

        let winnings;
        let ours;
        {
            let split = line.split('|').collect::<Vec<_>>();
            winnings = split[0];
            ours = split[1];
        }

        let winnings = winnings
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let ours = ours
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        results.push((winnings, ours));
    }
    results
}

fn count_points(winnings: &HashSet<usize>, ours: &HashSet<usize>) -> Option<usize> {
    let inter = winnings.intersection(&ours).collect::<Vec<_>>().len();
    if inter == 0 {
        return None;
    }
    Some(2_usize.pow(inter.saturating_sub(1) as u32))
}

mod part1 {
    use super::{count_points, parse_input};

    fn sum_winnings(s: &str) -> usize {
        parse_input(s)
            .into_iter()
            .filter_map(|(winnings, ours)| count_points(&winnings, &ours))
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use crate::day4::part1::sum_winnings;

        #[test]
        fn test_sum_winnings() {
            let test_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
            assert_eq!(sum_winnings(test_str), 13);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day4.txt").unwrap();
            dbg!(sum_winnings(&test_file));
        }
    }
}

mod part2 {
    use std::collections::HashMap;

    use super::{count_points, parse_input};

    fn calculate_total_cards(s: &str) -> usize {
        let parsed = parse_input(s);

        let mut sum = 0;
        let mut num_cards = vec![1; parsed.len()];

        for (game_index, game) in parsed.iter().enumerate() {
            let game_points = game.0.intersection(&game.1).collect::<Vec<_>>().len();
            let curr_card_num = num_cards[game_index];
            sum += curr_card_num;

            {
                let num_cards_len = num_cards.len();
                for fut_num_cards in &mut num_cards[num_cards_len.min(game_index + 1)
                    ..num_cards_len.min(game_index + game_points + 1)]
                {
                    *fut_num_cards += curr_card_num;
                }
            }
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        use crate::day4::part2::calculate_total_cards;

        #[test]
        fn test_count_games() {
            let test_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
            assert_eq!(calculate_total_cards(test_str), 30);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day4.txt").unwrap();
            dbg!(calculate_total_cards(&test_file));
        }
    }
}
