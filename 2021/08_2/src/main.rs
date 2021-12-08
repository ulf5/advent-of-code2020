use std::collections::HashSet;

fn main() {
    let sum: u32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let mut s = l.split('|');
            (
                s.next().unwrap().split_whitespace().collect(),
                s.next().unwrap().split_whitespace().collect(),
            )
        })
        .map(|(code, output): (Vec<&str>, Vec<&str>)| {
            let one = code.iter().find(|dig| dig.len() == 2).unwrap();
            let four = code.iter().find(|dig| dig.len() == 4).unwrap();
            let seven = code.iter().find(|dig| dig.len() == 3).unwrap();
            let eight = code.iter().find(|dig| dig.len() == 7).unwrap();
            let zero = code
                .iter()
                .find(|dig| {
                    dig.len() == 6
                        && !four.chars().all(|c| dig.contains(c))
                        && one.chars().all(|c| dig.contains(c))
                })
                .unwrap();
            let six = code
                .iter()
                .find(|dig| {
                    dig.len() == 6
                        && !four.chars().all(|c| dig.contains(c))
                        && !one.chars().all(|c| dig.contains(c))
                })
                .unwrap();
            let nine = code
                .iter()
                .find(|dig| dig.len() == 6 && dig != &zero && dig != &six)
                .unwrap();
            let three = code
                .iter()
                .find(|dig| dig.len() == 5 && one.chars().all(|c| dig.contains(c)))
                .unwrap();
            let five = code
                .iter()
                .find(|dig| dig.len() == 5 && dig.chars().all(|c| six.contains(c)))
                .unwrap();
            let two = code
                .iter()
                .find(|dig| dig.len() == 5 && dig != &five && dig != &three)
                .unwrap();

            let mut res = "".to_string();
            for dig in output {
                res.push(match char_set(&dig) {
                    s if s == char_set(zero) => '0',
                    s if s == char_set(one) => '1',
                    s if s == char_set(two) => '2',
                    s if s == char_set(three) => '3',
                    s if s == char_set(four) => '4',
                    s if s == char_set(five) => '5',
                    s if s == char_set(six) => '6',
                    s if s == char_set(seven) => '7',
                    s if s == char_set(eight) => '8',
                    s if s == char_set(nine) => '9',
                    x => {
                        dbg!(x);
                        panic!();
                    }
                });
            }
            res.parse::<u32>().unwrap()
        })
        .sum();
    dbg!(sum);
}

fn char_set(s: &str) -> HashSet<char> {
    s.chars().collect::<HashSet<char>>()
}
