fn main() {
    let mut times = 0;
    let depths: Vec<u32>  = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    for i in 0..depths.len() - 3 {
        if depths[i+1..i+4].iter().sum::<u32>() > depths[i..i+3].iter().sum() {
            times += 1;
        }
    }
    dbg!(times);
}
