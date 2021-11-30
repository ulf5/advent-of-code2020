use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct OneShotProgram {
    lines: Vec<String>,
}

impl OneShotProgram {
    fn from(lines: Vec<String>) -> Self {
        Self { lines }
    }

    fn run(&self) -> i32 {
        let mut cnt = 0;
        let mut executed = HashSet::new();
        let mut acc = 0;
        loop {
            if executed.get(&cnt).is_some() {
                break;
            }
            let s = &self.lines[cnt as usize];
            let mut sp = s.splitn(2, " ");
            executed.insert(cnt);
            let kalle = sp.next().unwrap();
            match kalle {
                "nop" => cnt += 1,
                "jmp" => cnt += sp.next().unwrap().parse::<i32>().unwrap(),
                "acc" => {
                    cnt += 1;
                    acc += sp.next().unwrap().parse::<i32>().unwrap();
                }
                _ => {
                    dbg!(kalle);
                    unreachable!();
                }
            };
        }

        acc
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        lines.push(line?);
    }
    let prog: OneShotProgram = OneShotProgram::from(lines);
    dbg!(prog.run());
    Ok(())
}
