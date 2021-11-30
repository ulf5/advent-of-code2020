fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = include_str!("../input.txt").lines();
    let target: u64 = lines.next().unwrap().parse()?;
    let min = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| {
            let xp: u64 = x.parse().unwrap();
            let mut tmp = 1;
            while tmp * xp < target {
                tmp += 1;
            }
            (xp, (tmp * xp) - target)
        })
        .min_by_key(|x| x.1)
        .unwrap();
    dbg!(target, min);

    dbg!(min.0 * min.1);
    Ok(())
}
