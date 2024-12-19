use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> (HashSet<&str>, Vec<&str>, usize) {
    let (left, right) = data.split_once("\n\n").unwrap();
    let stripes = left.trim().split(", ").collect::<HashSet<_>>();
    let max = stripes.iter().map(|s| s.len()).max().unwrap();
    let patterns = right.trim().lines().collect();
    (stripes, patterns, max)
}

fn find_order<'a>(memo: &mut HashMap<&'a str, usize>, stripes: &HashSet<&'a str>, pattern: &'a str, max: usize) -> usize {
    if pattern.is_empty() {
        return 1
    }
    if let Some(&v) = memo.get(pattern) {
        return v
    }
    let mut total = 0;
    for l in 1..=max.min(pattern.len()) {
        if stripes.contains(&pattern[..l]) {
            total += find_order(memo, stripes, &pattern[l..], max);
        }
    }
    memo.insert(pattern, total);
    total
}

pub fn run_one(data: &str) -> String {
    let (stripes, patterns, max) = parse(data);
    let mut memo = HashMap::new();
    patterns.iter().filter(|p| find_order(&mut memo, &stripes, p, max) > 0).count().to_string()
}

pub fn run_two(data: &str) -> String {
    let (stripes, patterns, max) = parse(data);
    let mut memo = HashMap::new();
    patterns.iter().map(|p| find_order(&mut memo, &stripes, p, max)).sum::<usize>().to_string()
}