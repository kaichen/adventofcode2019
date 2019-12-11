use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;

    for line in stdin.lock().lines() {
        let txt = line.unwrap();
        let my_int = txt.clone().parse::<u64>().unwrap();
        let answer = my_int / 3 - 2;
        sum = sum + answer;
    }

    println!("{}", sum);
}
