fn main() {
    let lines = include_str!("../input.txt").lines();
    let sum: u32 = lines.map(do_stuff).sum();
    dbg!(sum);
}

fn do_stuff(l: &str) -> u32 {
    let mut parts = l.split([':', ';']);
    parts.next().unwrap();

    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
    for part in parts {
        for color in part.split(',') {
            match color {
                green if green.ends_with(" green") => {
                    let g = green.trim().replace(" green", "").parse::<u32>().unwrap();
                    if g > max_green {
                        max_green = g;
                    }
                }
                blue if blue.ends_with(" blue") => {
                    let b = blue.trim().replace(" blue", "").parse::<u32>().unwrap();
                    if b > max_blue {
                        max_blue = b;
                    }
                }
                red if red.ends_with(" red") => {
                    let r = red.trim().replace(" red", "").parse::<u32>().unwrap();
                    if r > max_red {
                        max_red = r;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    max_red * max_blue * max_green
}
