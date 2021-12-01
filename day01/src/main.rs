type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let parsed = parse(input)?;

    let increases = sweeper(&parsed);
    println!("Part I answer: {}", increases);

    let increase_windows = sweeping_window(&parsed);
    println!("Part II answer: {}", increase_windows);
    Ok(())
}

fn parse(input: &str) -> Result<Vec<u64>> {
    input.lines().map(|line| line.parse::<u64>().map_err(|e| e.into())).collect::<Result<_>>()
}

fn sweeper(vec: &Vec<u64>) -> u64 {
    let mut count = 0;
    if let Some(mut ther) = vec.first() {
        for item in vec {
            if item > ther {
                count += 1;
            } else {
            }
            ther = item;
        }
    }
    count
}
fn sweeping_window(vec: &Vec<u64>) -> u64 {
    let mut count = 0;
    let mut prev_val = 0;
    let windows = vec.windows(3);

    for window in windows {
        let sum: u64 = window.iter().sum();

        if sum > prev_val {
            count += 1;
        }
        prev_val = sum;
    }

    // first run shouldn't count as per the requirements.
    count - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sweeper() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let increases = sweeper(&parsed);

        assert_eq!(increases, 7);

        Ok(())
    }

    #[test]
    fn test_sweeping_window() -> Result<()> {
        let str = include_str!("example.txt");
        let parsed = parse(str)?;
        let increases = sweeping_window(&parsed);

        assert_eq!(increases, 5);

        Ok(())
    }
}
