advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let list = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let end = list.len() - 1;
    let mut res = 0;
    for (i, a) in list.iter().enumerate() {
        let j = if i + 1 < end {
            i + 1
        } else {
            break;
        };
        for b in &list[j..end] {
            let sum = a + b;
            if sum == 2020 {
                res = a * b;
                break;
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let list = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let end = list.len() - 1;
    let mut res = 0;
    for i in 0..end - 2 {
        for j in i + 1..end - 1 {
            for k in j + 1..end {
                let sum = list[i] + list[j] + list[k];
                if sum == 2020 {
                    res = list[i] * list[j] * list[k];
                    break;
                }
            }
        }
    }
    Some(res)
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
