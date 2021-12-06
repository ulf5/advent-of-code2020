fn main() {
    let mut fish_per_age = [0u64; 9];
    include_str!("../input.txt")
        .lines()
        .map(|l| l.split(',').map(|e| e.parse::<usize>().unwrap()))
        .flatten()
        .for_each(|f| fish_per_age[f] += 1);

    for _ in 0..256 {
        let zero = fish_per_age[0];
        for age in 0..8 {
            fish_per_age[age] = fish_per_age[age + 1];
        }
        fish_per_age[6] += zero;
        fish_per_age[8] = zero;
    }
    let tot: u64 = fish_per_age.into_iter().sum();
    dbg!(tot);
}
