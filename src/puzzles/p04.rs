fn parse(data: &str) -> Vec<Vec<char>> {
    data.split("\n")
        .map(|s| s.bytes().map(|b| b as char).collect())
        .filter(|r: &Vec<_>| r.len() > 0)
        .collect::<Vec<Vec<_>>>()
}

pub fn run_one(data: &str) -> String {
    let grid = parse(data);
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if j + 3 < grid[i].len()
                && grid[i][j] == 'X'
                && grid[i][j + 1] == 'M'
                && grid[i][j + 2] == 'A'
                && grid[i][j + 3] == 'S'
            {
                sum += 1
            };
            if j >= 3
                && grid[i][j] == 'X'
                && grid[i][j - 1] == 'M'
                && grid[i][j - 2] == 'A'
                && grid[i][j - 3] == 'S'
            {
                sum += 1
            };
            if i + 3 < grid.len()
                && grid[i][j] == 'X'
                && grid[i + 1][j] == 'M'
                && grid[i + 2][j] == 'A'
                && grid[i + 3][j] == 'S'
            {
                sum += 1
            };
            if i >= 3
                && grid[i][j] == 'X'
                && grid[i - 1][j] == 'M'
                && grid[i - 2][j] == 'A'
                && grid[i - 3][j] == 'S'
            {
                sum += 1
            };
            if i >= 3
                && j >= 3
                && grid[i][j] == 'X'
                && grid[i - 1][j - 1] == 'M'
                && grid[i - 2][j - 2] == 'A'
                && grid[i - 3][j - 3] == 'S'
            {
                sum += 1
            };
            if i >= 3
                && j + 3 < grid[i].len()
                && grid[i][j] == 'X'
                && grid[i - 1][j + 1] == 'M'
                && grid[i - 2][j + 2] == 'A'
                && grid[i - 3][j + 3] == 'S'
            {
                sum += 1
            };
            if i + 3 < grid.len()
                && j >= 3
                && grid[i][j] == 'X'
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 2][j - 2] == 'A'
                && grid[i + 3][j - 3] == 'S'
            {
                sum += 1
            };
            if i + 3 < grid.len()
                && j + 3 < grid[i].len()
                && grid[i][j] == 'X'
                && grid[i + 1][j + 1] == 'M'
                && grid[i + 2][j + 2] == 'A'
                && grid[i + 3][j + 3] == 'S'
            {
                sum += 1;
            };
        }
    }
    sum.to_string()
}

pub fn run_two(data: &str) -> String {
    let grid = parse(data);
    let mut sum = 0;
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid.len() - 1) {
            if grid[i][j] == 'A' {
                let mut cross = 0;
                if grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S' {
                    cross += 1;
                }
                if grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M' {
                    cross += 1;
                }
                if grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S' {
                    cross += 1;
                }
                if grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M' {
                    cross += 1;
                }
                if cross == 2 {
                    sum += 1;
                }
            }
        }
    }
    sum.to_string()
}
