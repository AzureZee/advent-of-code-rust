advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let s = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let end = s.len() - 1;

    for i in 0..end - 1 {
        for j in i + 1..end {
            let sum = s[i] + s[j];
            if sum == 2020 {
                let res = s[i] * s[j];
                return Some(res);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let s = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let end = s.len() - 1;

    for i in 0..end - 2 {
        for j in i + 1..end - 1 {
            for k in j + 1..end {
                let sum = s[i] + s[j] + s[k];
                if sum == 2020 {
                    let res = s[i] * s[j] * s[k];
                    return Some(res);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
