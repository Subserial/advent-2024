use std::collections::{HashMap, HashSet, VecDeque};

const SKILL_OFFSET: usize = 1000;

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.trim().chars().collect()).collect()
}

fn char_loc(map: &Vec<Vec<char>>, find_c: char) -> (usize, usize) {
    map.iter().enumerate().find_map(|(row, v)| v.iter().enumerate().find_map(|(j, &c)| if c == find_c {Some((row, j))} else {None})).unwrap()
}

fn next_check(row: usize, col: usize, dir: char) -> Vec<(usize, usize, char)> {
    match dir {
        '^' => Vec::from([(row - 1, col, '^'), (row, col, '<'), (row, col, '>')]),
        '>' => Vec::from([(row, col, '^'), (row, col, 'v'), (row, col + 1, '>')]),
        '<' => Vec::from([(row, col, '^'), (row, col - 1, '<'), (row, col, 'v')]),
        'v' => Vec::from([(row, col, '<'), (row + 1, col, 'v'), (row, col, '>')]),
        _ => panic!(),
    }
}

fn run(map: &Vec<Vec<char>>) -> (usize, usize) {
    let (s_row, s_col) = char_loc(&map, 'S');
    let (e_row, e_col) = char_loc(&map, 'E');
    let mut seen = HashMap::new();
    let mut best = Vec::new();
    let mut queue = VecDeque::from([((s_row, s_col, '>'), (0, Vec::new()))]);
    while let Some(((c_row, c_col, c_dir), (weight, hist))) = queue.pop_front() {
        if let Some(&best_weight) = seen.get(&(c_row, c_col, c_dir)) {
            if weight > best_weight {
                continue
            }
            if c_row == e_row && c_col == e_col && weight < best_weight {
                best.clear();
            }
        }
        seen.insert((c_row, c_col, c_dir), weight);
        let mut next_hist = hist.clone();
        next_hist.push((c_row, c_col));
        if c_row == e_row && c_col == e_col {
            best.push(next_hist);
            continue
        }
        for (n_row, n_col, n_dir) in next_check(c_row, c_col, c_dir) {
            if map[n_row][n_col] != '#' {
                let n_weight = if c_dir != n_dir {
                    weight + SKILL_OFFSET
                } else {
                    weight + 1
                };
                queue.push_back(((n_row, n_col, n_dir), (n_weight, next_hist.clone())));
            }
        }
    }
    let length = seen.iter().filter(|(&(row, col, _), _)| row == e_row && col == e_col).map(|(_, &weight)| weight).min().unwrap();
    let all_seen = best.into_iter().flatten().collect::<HashSet<_>>().len();
    (length, all_seen)
}


pub fn run_one(data: &str) -> String {
    let map = parse(data);
    let (one, _) = run(&map);
    one.to_string()
}

pub fn run_two(data: &str) -> String {
    let map = parse(data);
    let (_, two) = run(&map);
    two.to_string()
}