use std::collections::HashMap;

fn parse(data: &str) -> (Vec<u32>, Vec<u32>) {
    data.lines()
        .map(|line| {
            let data = line.split_whitespace().take(2).collect::<Vec<_>>();
            (
                data[0].parse::<u32>().unwrap(),
                data[1].parse::<u32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>()
}

pub fn run_one(data: &str) -> String {
    let (mut left, mut right) = parse(data);
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let (left, right) = parse(data);
    let values = right.iter().fold(HashMap::new(), |mut map, val| {
        match map.get_mut(val) {
            None => {
                map.insert(*val, 1u32);
            }
            Some(count) => *count += 1,
        }
        map
    });
    left.iter()
        .map(|l| l * values.get(l).unwrap_or(&0u32))
        .sum::<u32>()
        .to_string()
}
