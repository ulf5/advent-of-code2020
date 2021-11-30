fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines: Vec<(char, i64)> = vec![];
    for line in include_str!("../input.txt").lines() {
        lines.push((line.chars().nth(0).unwrap(), line[1..].parse().unwrap()));
    }

    dbg!(xmas_run(lines));
    Ok(())
}

fn xmas_run(lines: Vec<(char, i64)>) -> i64 {
    let (mut x, mut y) = (0, 0);
    let (mut wx, mut wy) = (-1, 10);
    for (cmd, val) in lines.iter() {
        match cmd {
            'N' => {
                wx -= val;
            }
            'E' => {
                wy += val;
            }
            'S' => {
                wx += val;
            }
            'W' => {
                wy -= val;
            }
            'F' => {
                x += wx * val;
                y += wy * val;
            }
            'R' => {
                let (nx, ny) = new_wp((wx, wy), *val);
                wx = nx;
                wy = ny;
            }
            'L' => {
                let (nx, ny) = new_wp((wx, wy), 0 - val);
                wx = nx;
                wy = ny;
            }
            _ => {
                dbg!(cmd, val);
                unreachable!();
            }
        }
    }
    x.abs() + y.abs()
}

fn new_wp((wx, wy): (i64, i64), deg: i64) -> (i64, i64) {
    let steps = (deg / 90).rem_euclid(4);
    match steps {
        0 => (wx, wy),
        1 => (wy, -wx),
        2 => (-wx, -wy),
        3 => (-wy, wx),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
        let mut lines: Vec<(char, i64)> = vec![];
        let input = "F10
N3
F7
R90
F11";
        for line in input.lines() {
            lines.push((line.chars().nth(0).unwrap(), line[1..].parse().unwrap()));
        }
        assert_eq!(xmas_run(lines), 286);
    }
}
