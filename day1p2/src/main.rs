use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;

    for line in stdin.lock().lines() {
        let txt = line.unwrap();
        let my_int = txt.clone().parse::<u64>().unwrap();
        sum = sum + recursive_calc_fuel(my_int);
    }

    println!("{}", sum);
}

fn calcuate_fuel_requirement(input: u64) -> u64 {
    if input <= 6 {
        0
    } else {
        input / 3 - 2
    }
}

fn recursive_calc_fuel(input: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut req = input;
    loop {
        req = calcuate_fuel_requirement(req);
        sum = sum + req;
        if req == 0 {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::recursive_calc_fuel;

    #[test]
    fn test_sample() {
        assert_eq!(recursive_calc_fuel(14), 2);
        assert_eq!(recursive_calc_fuel(1969), 966);
        assert_eq!(recursive_calc_fuel(100756), 50346);
    }
}
