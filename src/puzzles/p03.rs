pub fn run_one(data: &str) -> String {
    let r_search = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    r_search
        .captures_iter(data)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let r_search = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    r_search
        .captures_iter(data)
        .map(|c| match c.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if enabled {
                    let (a, b) = (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str());
                    a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
                } else {
                    0
                }
            }
        })
        .sum::<u32>()
        .to_string()
}
