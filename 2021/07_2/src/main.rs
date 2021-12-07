fn main() {
    let crabs: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split(',').map(|e| e.parse::<i32>().unwrap()))
        .flatten()
        .collect();
    let max = crabs.iter().max().unwrap();

    let mut last_sum = 0;
    for i in 0..*max {
        let sum = crabs
            .iter()
            .map(|c| {
                let steps = (c - i).abs();
                steps * (steps + 1) / 2
            })
            .sum::<i32>();
        if i > 0 && sum > last_sum {
            dbg!(last_sum);
            break;
        }
        last_sum = sum;
    }
}
