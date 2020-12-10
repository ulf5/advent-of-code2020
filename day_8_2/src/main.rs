use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        lines.push(line?);
    }
    for i in 0..lines.len() {
        swap_jmp_nop(&mut lines, i);
        if let Some(acc) = run(&lines) {
            dbg!(acc);
            break;
        }
        swap_jmp_nop(&mut lines, i);
    }

    Ok(())
}
fn swap_jmp_nop(lines: &mut Vec<String>, i: usize) {
    if lines[i].starts_with("jmp") {
        lines[i] = lines[i].replace("jmp", "nop");
    } else if lines[i].starts_with("nop") {
        lines[i] = lines[i].replace("nop", "jmp");
    }
}

fn run(lines: &Vec<String>) -> Option<i32> {
    let mut cnt = 0;
    let mut executed = HashSet::new();
    let mut acc = 0;
    while lines.get(cnt as usize).is_some() {
        if executed.get(&cnt).is_some() {
            return None;
        }
        let s = &lines[cnt as usize];
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

    Some(acc)
}
