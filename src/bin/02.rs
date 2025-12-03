advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalids: Vec<u64> = Vec::new();
    for range in input.split(',') {
        let (min, max) = range.split_once('-').unwrap();
        let min = min.parse::<u64>().unwrap();
        let max = max.parse::<u64>().unwrap();

        for id in min..=max {
            let id_string = format!("{id}");
            let id_length = id_string.len();

            if id_length % 2 != 0 {
                continue;
            }

            let id_half = id_length / 2;
            let first_part = id_string[..id_half].to_string();
            let second_part = id_string[id_half..].to_string();

            if first_part == second_part {
                invalids.push(id);
            }
        }
    }

    let sum: u64 = invalids.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalids: Vec<u64> = Vec::new();
    for range in input.split(',') {
        let (min, max) = range.split_once('-').unwrap();
        let min = min.parse::<u64>().unwrap();
        let max = max.parse::<u64>().unwrap();

        for id in min..=max {
            let id_string = format!("{id}");
            let id_length = id_string.len();
            let mut invalid_found = false;

            for pattern_length in 1..=id_length / 2 {
                if invalid_found {
                    break;
                }

                // pattern_length must fit evenly in id_length
                if id_length % pattern_length != 0 {
                    continue;
                }

                let first_part = id_string[..pattern_length].to_string();

                let mut i = 0;
                while i < id_length {
                    let part_start = (1 + i) * pattern_length;
                    let part_end = part_start + pattern_length;
                    i += 1;

                    if part_end > id_length {
                        invalids.push(id);
                        invalid_found = true;
                        break;
                    }

                    let next_part = id_string[part_start..part_start + pattern_length].to_string();

                    if first_part != next_part {
                        break;
                    }
                }
            }
        }
    }

    let sum: u64 = invalids.iter().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
