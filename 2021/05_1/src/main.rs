use std::ops::RangeInclusive;

fn main() {
    let lines = include_str!("../input.txt").lines().map(|l| {
        let mut two_coors = l.splitn(2, " -> ").map(|c| {
            let mut s = c.splitn(2, ',').map(|d| d.parse::<usize>().unwrap());
            (s.next().unwrap(), s.next().unwrap())
        });
        (two_coors.next().unwrap(), two_coors.next().unwrap())
    });
    let mut grid = [[0u8; 1000]; 1000];
    for line in lines {
        if line.0 .0 == line.1 .0 {
            for y in inc_range(line.0 .1, line.1 .1) {
                grid[y][line.0 .0] += 1;
            }
        } else if line.0 .1 == line.1 .1 {
            for x in inc_range(line.0 .0, line.1 .0) {
                grid[line.0 .1][x] += 1;
            }
        }
    }
    let count = grid.iter().flatten().filter(|&x| *x >= 2).count();
    dbg!(count);
}

fn inc_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a >= b {
        b..=a
    } else {
        a..=b
    }
}
