fn main() {
    let pos: (i32, i32, i32) = include_str!("../input.txt")
        .lines()
        .map(|x| match x.split_once(" ") {
            Some(("forward", d)) => (d.parse::<i32>().unwrap(), 0, 0),
            Some(("up", d)) => (0, 0, -d.parse::<i32>().unwrap()),
            Some(("down", d)) => (0, 0, d.parse::<i32>().unwrap()),
            _ => panic!(),
        })
        .fold((0, 0, 0), |(a, b, c), (d, _, f)| {
            (a + d, b + (d * c), c + f)
        });
    dbg!(pos.0 * pos.1);
}
