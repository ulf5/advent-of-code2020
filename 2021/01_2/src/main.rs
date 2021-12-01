fn main() {
    let depths = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap());
    let count = depths
        .clone()
        .zip(depths.skip(3))
        .filter(|(cur, next)| next > cur)
        .count();
    dbg!(count);
}
