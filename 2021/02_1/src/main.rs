fn main() {
    let pos: (i32, i32) = include_str!("../input.txt")
        .lines()
        .map(|x| match x.split_once(" ") {
            Some(("forward", d)) => (d.parse::<i32>().unwrap(), 0),
            Some(("up", d)) => (0, -d.parse::<i32>().unwrap()),
            Some(("down", d)) => (0, d.parse::<i32>().unwrap()),
            _ => panic!(),
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();
    dbg!(pos.0 * pos.1);
}
