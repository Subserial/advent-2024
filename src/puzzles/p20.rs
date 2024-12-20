use std::collections::{HashMap, VecDeque};

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.trim().chars().collect()).collect()
}

fn char_loc(map: &Vec<Vec<char>>, find_c: char) -> (usize, usize) {
    map.iter().enumerate().find_map(|(row, v)| v.iter().enumerate().find_map(|(j, &c)| if c == find_c {Some((row, j))} else {None})).unwrap()
}

pub fn next_cheats(map: &Vec<Vec<char>>, row: usize, col: usize, dist: usize) -> Vec<(usize, usize, usize)> {
    let mut spots = Vec::new();
    for first in 1..=dist {
        for second in 0..=(dist - first) {
            if row + first < map.len() && col + second < map[0].len() {
                spots.push((row + first, col + second, first + second));
            }
            if row >= second && col + first < map[0].len() {
                spots.push((row - second, col + first, first + second));
            }
            if row >= first && col >= second {
                spots.push((row - first, col - second, first + second));
            }
            if row + second < map.len() && col >= first {
                spots.push((row + second, col - first, first + second));
            }
        }
    }
    spots
}

#[cfg(feature = "test-input")]
const CHEAT_QUALIFY: usize = 60;
#[cfg(not(feature = "test-input"))]
const CHEAT_QUALIFY: usize = 100;

fn run(map: &Vec<Vec<char>>, cheat_dist: usize) -> usize {
    let (s_row, s_col) = char_loc(&map, 'S');
    let (e_row, e_col) = char_loc(&map, 'E');
    let mut dists = HashMap::new();
    let mut queue = VecDeque::from([(s_row, s_col, 0usize)]);
    while let Some((c_row, c_col, dist)) = queue.pop_front() {
        if map[c_row][c_col] == '#' {
            continue
        }
        if let Some(&d) = dists.get(&(c_row, c_col)) {
            if d < dist {
                continue
            }
        }
        dists.insert((c_row, c_col), dist);
        if c_row == e_row && c_col == e_col {
            continue
        }
        if c_row > 0 {
            queue.push_back((c_row - 1, c_col, dist + 1));
        }
        if c_row < map.len() - 1 {
            queue.push_back((c_row + 1, c_col, dist + 1));
        }
        if c_col > 0 {
            queue.push_back((c_row, c_col - 1, dist + 1));
        }
        if c_row < map[0].len() - 1 {
            queue.push_back((c_row, c_col + 1, dist + 1));
        }
    }
    let mut good_cheats = 0;
    for (&(row, col), &dist) in &dists {
        for (f_row, f_col, f_dist) in next_cheats(&map, row, col, cheat_dist) {
            if let Some(&d) = dists.get(&(f_row, f_col)) {
                if d.saturating_sub(dist) >= CHEAT_QUALIFY + f_dist {
                    good_cheats += 1;
                }
            }
        }
    }
    good_cheats
}

pub fn run_one(data: &str) -> String {
    let map = parse(data);
    run(&map, 2).to_string()
}

pub fn run_two(data: &str) -> String {
    let map = parse(data);
    run(&map, 20).to_string()
}