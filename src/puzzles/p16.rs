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

fn is_junction(map: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut score = 0;
    if map[row - 1][col] != '#' {
        score += 1;
    }
    if map[row + 1][col] != '#' {
        score += 1;
    }
    if map[row][col - 1] != '#' {
        score += 1;
    }
    if map[row][col + 1] != '#' {
        score += 1;
    }
    score > 2
}

fn rev(c: char) -> char {
    match c {
        '^' => 'v',
        '>' => '<',
        '<' => '>',
        'v' => '^',
        _ => panic!(),
    }
}

fn run(map: &Vec<Vec<char>>) -> (usize, HashMap<(usize, usize), HashMap<(usize, usize, char), (char, usize)>>) {
    let (s_row, s_col) = char_loc(&map, 'S');
    let (e_row, e_col) = char_loc(&map, 'E');
    let mut paths: HashMap<(usize, usize), HashMap<(usize, usize, char), (char, usize)>> = HashMap::new();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([((s_row, s_col, '>'), (s_row, s_col, '>'), 0)]);
    while let Some(((l_row, l_col, l_dir), (c_row, c_col, c_dir), weight)) = queue.pop_front() {
        if l_row == c_row && l_col == c_col && l_dir == rev(c_dir) {
            continue
        }
        if (is_junction(&map, c_row, c_col) || (c_row == e_row && c_col == e_col)) && !(c_row == l_row && c_col == l_col) {
            match paths.get_mut(&(l_row, l_col)) {
                Some(v) => { v.insert((c_row, c_col, l_dir), (c_dir, weight)); },
                None => { paths.insert((l_row, l_col), HashMap::from([((c_row, c_col, l_dir), (c_dir, weight))])); },
            }
            match paths.get_mut(&(c_row, c_col)) {
                Some(v) => { v.insert((l_row, l_col, rev(c_dir)), (rev(l_dir), weight)); },
                None => { paths.insert((c_row, c_col), HashMap::from([((l_row, l_col, rev(c_dir)), (rev(l_dir), weight))])); },
            }
            if seen.contains(&(c_row, c_col)) {
                continue
            }
            seen.insert((c_row, c_col));
            for (n_row, n_col, n_dir) in next_check(c_row, c_col, c_dir) {
                if map[n_row][n_col] != '#' {
                    queue.push_back(((c_row, c_col, n_dir), (n_row, n_col, n_dir), 1));
                }
            }
        } else {
            for (n_row, n_col, n_dir) in next_check(c_row, c_col, c_dir) {
                if map[n_row][n_col] != '#' {
                    let n_weight = if c_dir != n_dir {
                        weight + SKILL_OFFSET
                    } else {
                        weight + 1
                    };
                    queue.push_back(((l_row, l_col, l_dir), (n_row, n_col, n_dir), n_weight));
                }
            }
        }
    }
    let mut seen = HashMap::from([((s_row, s_col), 0)]);
    let mut search = VecDeque::from([(s_row, s_col, '>', 0, vec![(s_row, s_col, '>')])]);
    while let Some((row, col, dir, weight, hist)) = search.pop_front() {
        for (&(n_row, n_col, n_dir), &(e_dir, n_weight)) in &paths[&(row, col)] {
            let total_weight = if dir != n_dir {
                weight + n_weight + SKILL_OFFSET
            } else {
                weight + n_weight
            };
            let prev_best = *seen.get(&(n_row, n_col)).unwrap_or(&usize::MAX);
            if prev_best < total_weight {
                continue
            }
            seen.insert((n_row, n_col), total_weight);
            let mut next_hist = hist.clone();
            next_hist.push((n_row, n_col, n_dir));
            if n_row == e_row && n_col == e_col {
                continue
            }
            search.push_back((n_row, n_col, e_dir, total_weight, next_hist));
        }
    }
    (seen[&(e_row, e_col)], paths)
}


pub fn run_one(data: &str) -> String {
    let map = parse(data);
    let (best, ..) = run(&map);
    best.to_string()
}

pub fn run_two(data: &str) -> String {
    let map = parse(data);
    let (best, paths) = run(&map);
    let (s_row, s_col) = char_loc(&map, 'S');
    let (e_row, e_col) = char_loc(&map, 'E');
    let mut best_paths = Vec::new();
    let mut search = VecDeque::from([(s_row, s_col, '>', 0, vec![(s_row, s_col, '>')])]);
    while let Some((row, col, dir, weight, hist)) = search.pop_front() {
        for (&(n_row, n_col, n_dir), &(e_dir, n_weight)) in &paths[&(row, col)] {
            let total_weight = if dir != n_dir {
                weight + n_weight + SKILL_OFFSET
            } else {
                weight + n_weight
            };
            if best < total_weight {
                continue
            }
            let mut next_hist = hist.clone();
            next_hist.push((n_row, n_col, n_dir));
            if n_row == e_row && n_col == e_col {
                best_paths.push(next_hist);
                continue
            }
            search.push_back((n_row, n_col, e_dir, total_weight, next_hist));
        }
    }
    for bp in &best_paths {
        let mut buf = map.clone();
        for &(row, col, _) in bp {
            buf[row][col] = 'O';
        }
        for line in buf {
            println!("{}", line.iter().collect::<String>());
        }
        println!();
    }


    let mut seen = HashSet::<()>::new();



    seen.len().to_string()
}