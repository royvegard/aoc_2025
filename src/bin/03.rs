advent_of_code::solution!(3);

fn get_joltage2(bank: &str) -> usize {
    let mut sorted_bank: Vec<(usize, usize)> = Vec::new();
    for (index, battery) in bank.chars().enumerate() {
        let battery = battery.to_string().parse::<usize>().unwrap();
        sorted_bank.push((battery, index));
    }
    sorted_bank.sort_by_key(|k| k.0);
    sorted_bank.reverse();

    let battery_1 = sorted_bank[0];
    let battery_2 = sorted_bank[1];

    if battery_1.0 == battery_2.0 || battery_1.1 < battery_2.1 {
        return battery_1.0 * 10 + battery_2.0;
    } else if battery_1.1 > battery_2.1 && battery_1.1 == sorted_bank.len() - 1 {
        return battery_2.0 * 10 + battery_1.0;
    } else {
        // find the next battery that has a higher index than first_battery
        for j in 0..sorted_bank.len() {
            if sorted_bank[j].1 > battery_1.1 {
                return battery_1.0 * 10 + sorted_bank[j].0;
            }
        }
    }
    0
}

fn get_joltage12(bank: &str) -> usize {
    let bat: Vec<i32> = bank
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let mut batteries: Vec<isize> = vec![-1];

    for b in 1..12 + 1 {
        batteries.push(batteries[b - 1] + 1);

        for i in batteries[b]..(bat.len() - 12 + b) as isize {
            if bat[i as usize] > bat[batteries[b] as usize] {
                batteries[b] = i;
            }
        }
    }
    batteries.remove(0);

    let mut on_batteries: Vec<i32> = batteries.iter().map(|i| bat[*i as usize]).collect();

    on_batteries.reverse();
    let mut joltage = 0;
    for i in 0..on_batteries.len() {
        let factor = 10usize.pow(i as u32);
        joltage += on_batteries[i] as usize * factor;
    }
    joltage
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut total_joltage = 0;
    for bank in input.lines() {
        total_joltage += get_joltage2(bank);
    }
    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut total_joltage = 0;
    for bank in input.lines() {
        total_joltage += get_joltage12(bank);
    }
    Some(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
