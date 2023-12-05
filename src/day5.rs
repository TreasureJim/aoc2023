#![allow(unused)]

use std::{cmp::Ordering, ops::Range};

struct RangeMap {
    ranges: Vec<(Range<usize>, isize)>,
}

impl RangeMap {
    pub fn new(mut key_values: Vec<(Range<usize>, isize)>) -> Self {
        key_values.sort_by(|(range1, _), (range2, _)| RangeMap::cmp_range_range(range1, range2));
        Self { ranges: key_values }
    }

    pub fn get_key(&self, key: usize) -> Option<isize> {
        let found = self
            .ranges
            .binary_search_by(|vec| RangeMap::cmp_range_num(&vec.0, &key))
            .ok();
        if found.is_none() {
            return None;
        }
        Some(self.ranges[found.unwrap()].1)
    }

    pub fn get_new_value_from_map(&self, key: usize) -> usize {
        let x = key as isize + self.get_key(key).unwrap_or(0);
        return x as usize;
    }

    fn cmp_range_num(range: &Range<usize>, compare: &usize) -> Ordering {
        if range.contains(&compare) {
            return Ordering::Equal;
        } else if range.start < *compare {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }

    fn cmp_range_range(range1: &Range<usize>, range2: &Range<usize>) -> Ordering {
        if range1.end <= range2.start {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

struct Mapping {
    seed_to_soil: RangeMap,
    soil_to_fert: RangeMap,
    fert_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temp: RangeMap,
    temp_to_humid: RangeMap,
    humid_to_loc: RangeMap,
}

impl Mapping {
    pub fn parse_input(s: &[&[&str]]) -> Self {
        let mut section_iter = s.into_iter();
        // dest src length

        let seed_to_soil;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            seed_to_soil = RangeMap::new(key_values)
        }

        let soil_to_fert;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            soil_to_fert = RangeMap::new(key_values)
        }

        let fert_to_water;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            fert_to_water = RangeMap::new(key_values)
        }

        let water_to_light;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            water_to_light = RangeMap::new(key_values)
        }

        let light_to_temp;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            light_to_temp = RangeMap::new(key_values)
        }

        let temp_to_humid;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            temp_to_humid = RangeMap::new(key_values)
        }

        let humid_to_loc;
        {
            let key_values = Mapping::parse_section(&section_iter.next().unwrap()[1..]);
            humid_to_loc = RangeMap::new(key_values)
        }

        Self {
            seed_to_soil,
            soil_to_fert,
            fert_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humid,
            humid_to_loc,
        }
    }

    /// Accepts array of lines.
    fn parse_section(lines: &[&str]) -> Vec<(Range<usize>, isize)> {
        lines
            .iter()
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

    pub fn get_loc(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.get_new_value_from_map(seed);
        let fert = self.soil_to_fert.get_new_value_from_map(soil);

        let water = self.fert_to_water.get_new_value_from_map(fert);
        let light = self.water_to_light.get_new_value_from_map(water);
        let temp = self.light_to_temp.get_new_value_from_map(light);
        let humid = self.temp_to_humid.get_new_value_from_map(temp);
        let loc = self.humid_to_loc.get_new_value_from_map(humid);

        return loc;
    }
}

mod part1 {
    use super::Mapping;

    fn get_lowest_loc(s: &str) -> usize {
        let paragraphs: Vec<_> = s
            .split_terminator("\n\n") // Split into paragraphs
            .map(|paragraph| paragraph.lines().collect::<Vec<_>>().into_boxed_slice()) // Collect into slice
            .collect();

        // seeds
        let seeds = paragraphs[0][0].split(": ").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let map = Mapping::parse_input(
            &paragraphs[1..]
                .iter()
                .map(|boxed_slice| boxed_slice.as_ref())
                .collect::<Vec<_>>(),
        );
        let locs = seeds.iter().map(|x| map.get_loc(*x)).collect::<Vec<_>>();
        locs.into_iter().min().unwrap()
    }

    mod tests {
        use crate::day5::part1::get_lowest_loc;

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
            assert_eq!(get_lowest_loc(test_str), 35);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day5.txt").unwrap();
            dbg!(get_lowest_loc(&test_file));
        }
    }
}

mod part2 {
    use super::*;

    fn get_lowest_loc(s: &str) -> usize {
        let paragraphs: Vec<_> = s
            .split_terminator("\n\n") // Split into paragraphs
            .map(|paragraph| paragraph.lines().collect::<Vec<_>>().into_boxed_slice()) // Collect into slice
            .collect();

        // seeds
        let seeds = paragraphs[0][0].split(": ").collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let seeds: Vec<usize> = seeds.chunks(2).map(|window| window[0]..window[0]+window[1]).flat_map(|range| range.collect::<Vec<_>>()).collect();

        let map = Mapping::parse_input(
            &paragraphs[1..]
                .iter()
                .map(|boxed_slice| boxed_slice.as_ref())
                .collect::<Vec<_>>(),
        );
        let locs = seeds.iter().map(|x| map.get_loc(*x)).collect::<Vec<_>>();
        locs.into_iter().min().unwrap()
    }

    mod tests {
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
            assert_eq!(get_lowest_loc(test_str), 46);
        }

        #[test]
        fn test_final() {
            let test_file = std::fs::read_to_string("day5.txt").unwrap();
            dbg!(get_lowest_loc(&test_file));
        }
    }
}
