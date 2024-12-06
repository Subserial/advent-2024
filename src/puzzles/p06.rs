use std::collections::HashSet;

fn parse(data: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let map = data
        .lines()
        .map(|line| line.bytes().map(|b| b as char).collect())
        .collect::<Vec<Vec<_>>>();
    let pos = map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find(|(_, &c)| c == '^')
                .map(|(j, _)| (j, i))
        })
        .next()
        .unwrap();
    (map, pos)
}

fn next_step(pos: (usize, usize), dir: char, rows: usize, cols: usize) -> Option<(usize, usize)> {
    match dir {
        '^' if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        '>' if pos.0 + 1 < cols => Some((pos.0 + 1, pos.1)),
        '<' if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        'V' if pos.1 + 1 < rows => Some((pos.0, pos.1 + 1)),
        _ => None,
    }
}

fn next_dir(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'V',
        'V' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}

pub fn run_one(data: &str) -> String {
    let (map, mut pos) = parse(data);
    let mut seen = HashSet::new();
    let mut dir = '^';
    let rows = map.len();
    let cols = map.first().unwrap().len();
    loop {
        seen.insert(pos);
        let Some(next_pos) = next_step(pos, dir, rows, cols) else {
            break;
        };
        if map[next_pos.1][next_pos.0] == '#' {
            dir = next_dir(dir);
        } else {
            pos = next_pos;
        }
    }
    seen.len().to_string()
}

fn will_loop(map: &Vec<Vec<char>>, start: (usize, usize), block: (usize, usize)) -> bool {
    let mut pos = start;
    let mut seen = HashSet::<((usize, usize), char)>::new();
    let mut dir = '^';
    let rows = map.len();
    let cols = map.first().unwrap().len();
    loop {
        if !seen.insert((pos, dir)) {
            return true;
        }
        let Some(next_pos) = next_step(pos, dir, rows, cols) else {
            return false;
        };
        if next_pos == block || map[next_pos.1][next_pos.0] == '#' {
            dir = next_dir(dir);
        } else {
            pos = next_pos;
        }
    }
}

pub fn run_two(data: &str) -> String {
    let (map, pos) = parse(data);
    let rows = map.len();
    let cols = map.first().unwrap().len();
    let mut good = 0;
    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == '.' {
                if will_loop(&map, pos, (j, i)) {
                    good += 1;
                }
            }
        }
    }
    good.to_string()
}
