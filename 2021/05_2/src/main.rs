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
        } else {
            for point in point_range(line.0, line.1) {
                grid[point.1][point.0] += 1;
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

fn point_range(a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
    let x_range = inc_range(a.0, b.0);
    let y_range = inc_range(a.1, b.1);
    if a.0 >= b.0 && a.1 >= b.1 {
        x_range.zip(y_range).collect()
    } else if a.0 >= b.0 {
        x_range.zip(y_range.rev()).collect()
    } else if a.1 >= b.1 {
        x_range.rev().zip(y_range).collect()
    } else {
        x_range.rev().zip(y_range.rev()).collect()
    }
}
