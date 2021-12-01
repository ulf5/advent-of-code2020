fn main() {
    let mut times = 0;
    let depths = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap());
    depths
        .clone()
        .zip(depths.skip(3))
        .for_each(|(cur, next)| {
            if next > cur {
                times += 1
            }
        });
    dbg!(times);
}
