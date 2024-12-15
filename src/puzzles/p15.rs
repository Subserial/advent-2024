fn parse(data: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (map, seq) = data.split_once("\n\n").unwrap();
    (
        map.lines().map(|l| l.trim().chars().collect()).collect(),
        seq.lines().map(|s| s.chars()).flatten().collect(),
    )
}

fn dir_next(dir: char, row: usize, col: usize) -> (usize, usize) {
    match dir {
        '^' => (row - 1, col),
        'v' => (row + 1, col),
        '<' => (row, col - 1),
        '>' => (row, col + 1),
        _ => panic!(),
    }
}

fn can_push(map: &mut Vec<Vec<char>>, row: usize, col: usize, dir: char) -> bool {
    match map[row][col] {
        '.' => true,
        '#' => false,
        'O' | '@' => {
            let (row_next, col_next) = dir_next(dir, row, col);
            can_push(map, row_next, col_next, dir)
        }
        '[' => {
            let (row_left, col_left) = dir_next(dir, row, col);
            let (row_right, col_right) = dir_next(dir, row, col + 1);
            match dir {
                '^' | 'v' => {
                    can_push(map, row_left, col_left, dir)
                        && can_push(map, row_right, col_right, dir)
                }
                '<' => can_push(map, row_left, col_left, dir),
                '>' => can_push(map, row_right, col_right, dir),
                _ => panic!(),
            }
        }
        ']' => {
            let (row_left, col_left) = dir_next(dir, row, col - 1);
            let (row_right, col_right) = dir_next(dir, row, col);
            match dir {
                '^' | 'v' => {
                    can_push(map, row_left, col_left, dir)
                        && can_push(map, row_right, col_right, dir)
                }
                '<' => can_push(map, row_left, col_left, dir),
                '>' => can_push(map, row_right, col_right, dir),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn push(map: &mut Vec<Vec<char>>, row: usize, col: usize, dir: char) {
    match map[row][col] {
        '.' | '#' => (),
        'O' | '@' => {
            let (row_next, col_next) = dir_next(dir, row, col);
            push(map, row_next, col_next, dir);
            map[row_next][col_next] = map[row][col];
            map[row][col] = '.';
        }
        '[' => {
            let (row_left, col_left) = dir_next(dir, row, col);
            if let '<' | '>' = dir {
                push(map, row_left, col_left, dir);
                map[row_left][col_left] = map[row][col];
                map[row][col] = '.';
            } else {
                let (row_right, col_right) = dir_next(dir, row, col + 1);
                push(map, row_left, col_left, dir);
                push(map, row_right, col_right, dir);
                map[row_left][col_left] = map[row][col];
                map[row_right][col_right] = map[row][col + 1];
                map[row][col] = '.';
                map[row][col + 1] = '.';
            }
        }
        ']' => {
            let (row_right, col_right) = dir_next(dir, row, col);
            if let '<' | '>' = dir {
                push(map, row_right, col_right, dir);
                map[row_right][col_right] = map[row][col];
                map[row][col] = '.';
            } else {
                let (row_left, col_left) = dir_next(dir, row, col - 1);
                push(map, row_left, col_left, dir);
                push(map, row_right, col_right, dir);
                map[row_left][col_left] = map[row][col - 1];
                map[row_right][col_right] = map[row][col];
                map[row][col - 1] = '.';
                map[row][col] = '.';
            }
        }
        _ => panic!(),
    }
}

fn score(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter().enumerate().filter_map(move |(j, &c)| {
                if let 'O' | '[' = c {
                    Some(i * 100 + j)
                } else {
                    None
                }
            })
        })
        .flatten()
        .sum()
}

pub fn run_one(data: &str) -> String {
    let (mut map, seq) = parse(data);
    let (mut row, mut col) = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &c)| if c == '@' { Some((i, j)) } else { None })
        })
        .unwrap();
    for action in seq {
        if can_push(&mut map, row, col, action) {
            push(&mut map, row, col, action);
            (row, col) = dir_next(action, row, col);
        }
    }
    score(&map).to_string()
}

pub fn run_two(data: &str) -> String {
    let (mut map, seq) = parse(data);
    map = map
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!(),
                })
                .flatten()
                .collect()
        })
        .collect();
    let (mut row, mut col) = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &c)| if c == '@' { Some((i, j)) } else { None })
        })
        .unwrap();
    for action in seq {
        if can_push(&mut map, row, col, action) {
            push(&mut map, row, col, action);
            (row, col) = dir_next(action, row, col);
        }
    }
    score(&map).to_string()
}
