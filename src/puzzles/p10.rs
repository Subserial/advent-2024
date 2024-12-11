use std::collections::HashSet;

fn parse(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn rating(map: &Vec<Vec<u32>>, collector: impl Fn(Vec<(usize, usize)>) -> usize) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 0)
                .map(move |(j, _)| (i, j))
        })
        .flatten()
        .map(|start| {
            let mut heads = vec![start];
            let mut height = 1;
            while height <= 9 && heads.len() > 0 {
                let mut next = vec![];
                for (row, col) in heads {
                    if col > 0 && map[row][col - 1] == height {
                        next.push((row, col - 1));
                    }
                    if col + 1 < map[0].len() && map[row][col + 1] == height {
                        next.push((row, col + 1));
                    }
                    if row > 0 && map[row - 1][col] == height {
                        next.push((row - 1, col));
                    }
                    if row + 1 < map.len() && map[row + 1][col] == height {
                        next.push((row + 1, col));
                    }
                }
                heads = next;
                height += 1;
            }
            collector(heads)
        })
        .sum()
}

pub fn run_one(data: &str) -> String {
    let map = parse(data);
    rating(&map, |nines| {
        nines.into_iter().collect::<HashSet<_>>().len()
    })
    .to_string()
}

pub fn run_two(data: &str) -> String {
    let map = parse(data);
    rating(&map, |nines| nines.len()).to_string()
}
