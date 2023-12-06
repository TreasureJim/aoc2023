#![allow(unused)]

use std::{cmp::Ordering, ops::Range};

/*
*
* NOT WORKING
* Solution inspired from: https://github.com/womogenes/AoC-2023-Solutions/blob/main/day_05/day_05_p2.py#L13
*
*/

fn range_contains_range<T: PartialOrd>(this: &Range<T>, other: &Range<T>) -> bool {
    this.start <= other.start && this.end >= other.end
}

fn remap(range: &Range<usize>, layer: &[(Range<usize>, isize)]) -> Vec<Range<usize>> {
    let mut answer = Vec::new();

    for (layer_range, offset) in layer {
        // if layer_range contains range
        // if not (end < lo or src > hi):
        if !(layer_range.end < range.start || layer_range.start > range.end) {
            answer.push((
                layer_range.start.max(range.start)..layer_range.end.min(range.end),
                *offset,
            ));
        }
    }

    // check if there are any spacing
    {
        let mut temp: Vec<(Range<usize>, isize)> = Vec::new();
        for window in answer.windows(2) {
            let (ans_range, _) = &window[0];
            let (ans_range2, _) = &window[1];

            if ans_range.end <= ans_range.start {
                temp.push((ans_range.end..ans_range2.start, 0));
            }
        }
        answer.append(&mut temp);
    }

    if answer.len() == 0 {
        return vec![range.clone()];
    }

    let first_start = answer.first().unwrap().0.start;
    if first_start > range.start {
        answer.push((range.start..first_start, 0))
    }
    let last_finish = answer.last().unwrap().0.end;
    if last_finish < range.end {
        answer.push((last_finish..range.end, 0));
    }

    answer
        .iter()
        .map(|(range, offset)| {
            (range.start as isize + offset) as usize..(range.end as isize + offset) as usize
        })
        .collect()
}

/// Accepts array of lines.
fn parse_section(lines: &str) -> Vec<(Range<usize>, isize)> {
    lines
        .lines()
        .skip(1)
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (
                nums[1]..nums[1] + nums[2],
                nums[0] as isize - nums[1] as isize,
            )
        })
        .collect::<Vec<_>>()
}

fn get_lowest_loc(seeds: Vec<Range<usize>>, maps: Vec<Vec<(Range<usize>, isize)>>) -> usize {
    let mut lowest = usize::MAX;

    for seed in seeds.into_iter() {
        let mut cur_ranges = vec![seed];
        let mut new_ranges = Vec::new();

        for map in maps.iter() {
            for range in cur_ranges.iter() {
                new_ranges.append(&mut remap(range, map));
            }

            cur_ranges = new_ranges;
            new_ranges = vec![];
        }

        lowest = lowest.min(cur_ranges.iter().map(|x| x.start).min().unwrap())
    }

    lowest
}

mod part1 {
    use super::{get_lowest_loc, parse_section};

    fn part1_loc(s: &str) -> usize {
        let paragraphs: Vec<_> = s.split_terminator("\n\n").collect();

        // seeds
        let seeds = paragraphs[0].split(": ").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|x| {
                let x = x.parse::<usize>().unwrap();
                x..x + 1
            })
            .collect::<Vec<_>>();

        let maps = paragraphs[1..]
            .iter()
            .map(|para| parse_section(para))
            .collect();

        get_lowest_loc(seeds, maps)
    }

    mod tests {
        use crate::day5::part1::{get_lowest_loc, part1_loc};

        #[test]
        fn test_whole_str() {
            let test_str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
            assert_eq!(part1_loc(test_str), 35);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day5.txt").unwrap();
            dbg!(part1_loc(&test_file));
        }
    }
}

mod part2 {
    use super::*;

    fn part2_loc(s: &str) -> usize {
        let paragraphs: Vec<_> = s.split_terminator("\n\n").collect();

        // seeds
        let seeds = paragraphs[0].split(": ").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|x| x[0]..x[0] + x[1])
            .collect::<Vec<_>>();

        let maps = paragraphs[1..]
            .iter()
            .map(|para| parse_section(para))
            .collect();

        get_lowest_loc(seeds, maps)
    }

    mod tests {
        use crate::day5::part2::{self, part2_loc};

        use super::get_lowest_loc;

        #[test]
        fn test_whole_str() {
            let test_str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
            assert_eq!(part2_loc(test_str), 46);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day5.txt").unwrap();
            dbg!(part2_loc(&test_file));
        }
    }
}
