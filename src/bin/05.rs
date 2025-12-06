use std::cmp::max;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Range {
    start: usize,
    stop: usize,
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    ranges.sort_by_key(|r| r.start);

    loop {
        let mut done = true;
        for i in 0..ranges.len() - 1 {
            if ranges[i].stop >= ranges[i + 1].start {
                ranges[i].stop = max(ranges[i].stop, ranges[i + 1].stop);
                ranges.remove(i + 1);
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }
}

fn check_ingredient(ingredient: usize, ranges: &Vec<Range>) -> bool {
    for range in ranges {
        if ingredient >= range.start && ingredient <= range.stop {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges: Vec<Range> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();
    let mut parsing_ranges = true;

    for line in input.lines() {
        if parsing_ranges {
            if line.is_empty() {
                parsing_ranges = false;
                continue;
            }

            let range = line.split_once('-').unwrap();
            ranges.push(Range {
                start: range.0.parse().unwrap(),
                stop: range.1.parse().unwrap(),
            });
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    merge_ranges(&mut ranges);

    let mut fresh_ingredients = 0;
    for ingredient in ingredients {
        if check_ingredient(ingredient, &ranges) {
            fresh_ingredients += 1;
        }
    }

    Some(fresh_ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<Range> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let range = line.split_once('-').unwrap();
        ranges.push(Range {
            start: range.0.parse().unwrap(),
            stop: range.1.parse().unwrap(),
        });
    }

    merge_ranges(&mut ranges);

    let mut fresh_ids = 0;
    for range in ranges {
        fresh_ids += range.stop - range.start + 1;
    }

    Some(fresh_ids as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
