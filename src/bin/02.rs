advent_of_code::solution!(2);
// example
// ["1", "3", "a", "abcde"]
// ["1", "3", "b", "cdefg"]
// ["2", "9", "c", "ccccccccc"]

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| {
            line.split(&['-', ':', ' '][..])
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let s: Vec<Vec<&str>> = parse(input);
    let mut result = 0;
    for v in s {
        let (min, max) = (
            v[0].parse::<usize>().unwrap(),
            v[1].parse::<usize>().unwrap(),
        );
        let range = min..=max;
        let count = v[3].matches(v[2]).count();
        if range.contains(&count) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let s: Vec<Vec<&str>> = parse(input);
    let mut result = 0;
    for v in s {
        let (idx_1, idx_2) = (
            v[0].parse::<usize>().unwrap(),
            v[1].parse::<usize>().unwrap(),
        );
        let count = v[3]
            .match_indices(v[2])
            .filter(|(i, _)| *i + 1 == idx_1 || *i + 1 == idx_2)
            .count();
        if count == 1 {
            result += 1;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
