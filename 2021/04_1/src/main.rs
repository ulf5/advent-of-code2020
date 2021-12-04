use std::collections::BTreeMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let calls = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap());

    let mut boards = vec![];
    let mut cnt = 0usize;
    for l in lines {
        if l.trim() == "" {
            boards.push((BTreeMap::new(), [0; 10]));
            cnt = 0;
        } else {
            l.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(i, e)| {
                    boards.last_mut().unwrap().0.insert(e, (i, cnt));
                });
            cnt += 1;
        }
    }

    let mut called = vec![];
    'game: for call in calls {
        called.push(call);
        for mut board in boards.iter_mut() {
            if let Some((x, y)) = board.0.get(&call) {
                board.1[*x] += 1;
                board.1[5 + *y] += 1;
                if board.1[5 + *y] == 5 || board.1[*x] == 5 {
                    let tot: usize = board
                        .0
                        .iter()
                        .map(|(&c, _)| c)
                        .filter(|c| !called.contains(c))
                        .sum();
                    dbg!(tot * call);

                    break 'game;
                }
            }
        }
    }
}
