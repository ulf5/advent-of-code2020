fn main() {
    let initial_fish: Vec<u8> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split(',').map(|e| e.parse::<u8>().unwrap()))
        .flatten()
        .collect();
    let mut fish = initial_fish;
    let mut added = vec![];

    for _ in 0..=80 {
        let mut next_added = vec![];
        fish = fish
            .iter()
            .chain(added.iter())
            .map(|&x| {
                if x == 0 {
                    next_added.push(8u8);
                    6u8
                } else {
                    x - 1
                }
            })
            .collect();
        added = next_added;
    }
    dbg!(fish.len());
}
