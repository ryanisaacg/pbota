use std::env;
use std::io::{self, BufRead};
use pbota_core::moves;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut prompt = |msg| {
        println!("{msg}");
        iterator.next().unwrap().unwrap()
    };

    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "move" => {
            let stat = prompt("What stat are you rolling? (Leave blank for none)");
            let preamble = prompt("What's the move's preamble?");
            let mut options = Vec::new();
            loop {
                let range = prompt("On a... (Leave blank for done)");
                if range.is_empty() {
                    break;
                }
                let match_range = matcher(&range[..]);
                let text = prompt("...");
                options.push((match_range, format!("On a {range}, {text}")));
            }
            let postamble = prompt("What's the move's postamble?");
            let stat = if stat.chars().next() == Some('-') {
                Some((stat[1..].to_owned(), -1))
            } else if stat.chars().next() == Some('+') {
                Some((stat[1..].to_owned(), 1))
            } else {
                Some((stat, 1))
            };
            let move_val = moves::Move {
                preamble: Some(preamble), // TODO
                postamble: Some(postamble), // TODO
                options,
                stat,
            };
            let data = serde_json::to_string(&move_val).unwrap();
            println!("{data}");
        }
        _ => {}
    }
}

fn matcher(line: &str) -> moves::Matcher {
    let (first, line) = num(line);
    if line.chars().next() == Some('+') {
        moves::Matcher::Greater(first)
    } else if line.len() > 1 {
        let (second, _) = num(&line[1..]);
        moves::Matcher::Range(first, second)
    } else {
        moves::Matcher::Less(first)
    }
}

fn num(line: &str) -> (i32, &str) {
    let mut num = 0i32;
    for (idx, chr) in line.chars().enumerate() {
        match char::to_digit(chr, 10) {
            Some(digit) => {
                num = 10 * num + (digit as i32);
            }
            None => {
                return (num, &line[idx..]);
            }
        }
    }

    (num, "")
}
