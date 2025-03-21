// TODO optimize memory usage
use rayon::prelude::*;
use std::{collections::HashSet, io, process};

fn main() {
    let inp = read_input();
    let input = parse_input(inp);
    let primes = collect_primes(input.0, input.1);
    let factors: HashSet<(u64, u64, u64)> = factorize(primes);
    // TODO sort factors (glidesort?)

    println!("{:?}", factors.len());

    // TODO bottleneck -> use BufWriter
    // for factor in factors {
    //     println!("{:?}", factor);
    // }
}

fn read_input() -> String {
    println!("Enter range [u64 u64]:");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Unable to read input");

    inp.trim().to_string()
}

fn parse_input(input: String) -> (u64, u64) {
    // split input to format (u64, u64)
    let split_input: Vec<&str> = input.split_whitespace().collect();

    if split_input.len() != 2 {
        eprintln!("2 inputs needed: 'start' and 'end'");
        process::exit(1);
    }

    let parsed_input: Vec<u64> = split_input
        .iter()
        .map(|i| {
            i.parse::<u64>().unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            })
        })
        .collect();

    let tuple_input = (parsed_input[0], parsed_input[1]);
    tuple_input
}

trait Prime {
    fn prime(self) -> bool;
}

impl Prime for u64 {
    // check if number is prime
    fn prime(self) -> bool {
        // base cases
        if self < 2 {
            return false;
        }
        if self == 2 || self == 3 {
            return true;
        }
        if self % 2 == 0 || self % 3 == 0 {
            return false;
        }

        // if a number n is not prime, it must have at least one pair of factors:
        // n=a×b, where a and b are factors of n
        // if both a and b were greater than √n, their product would be greater than n, which is a contradiction
        // so, at least one of the factors must be ≤ √n
        // if we don’t find any factors up to √n, there can’t be any beyond it (since they would be paired with a factor already checked)
        // checking all numbers up to n-1, is O(n) time complexity
        // stopping at √n reduces it to O(√n)
        let limit = (self as f64).sqrt() as u64;

        (5..=limit)
            .step_by(6) // all primes >3 are of the form 6k ± 1
            .collect::<Vec<u64>>()
            .par_iter()
            .all(|&i| self % i != 0 && self % (i + 2) != 0)
    }
}

fn collect_primes(start: u64, end: u64) -> Vec<u64> {
    // filter out non prime numbers
    (start..=end)
        .into_par_iter()
        .filter(|&n| n.prime())
        .collect()
}

fn factorize(primes: Vec<u64>) -> HashSet<(u64, u64, u64)> {
    // calculate all prime factors
    primes
        .par_iter()
        .flat_map(|&num1| {
            primes
                .par_iter()
                .filter_map(|&num2| {
                    // only store (a * b, min(a, b), max(a, b)) to avoid redundant pairs like (6,2,3) and (6,3,2)
                    // this ensures each unique pair appears only once.
                    if num1 < num2 {
                        Some((num1 * num2, num1, num2))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_prime() {
        let primes: [u64; 25] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        assert!(primes.into_par_iter().all(|x| x.prime()));
    }

    #[test]
    fn no_prime() {
        let non_primes: [u64; 25] = [
            4, 6, 8, 10, 44, 46, 410, 412, 56, 512, 64, 68, 610, 74, 76, 710, 86, 812, 94, 104,
            106, 1012, 116, 1112, 1210,
        ];

        assert!(!non_primes.into_par_iter().all(|x| x.prime()));
    }

    #[test]
    fn collect_prime() {
        let primes: [u64; 25] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        assert_eq!(Vec::from(primes), collect_primes(0, 100));
    }

    #[test]
    fn factorized() {
        let primes: Vec<u64> = vec![2, 3, 5, 7];
        let factors: HashSet<(u64, u64, u64)> = HashSet::from([
            (6, 2, 3),
            (10, 2, 5),
            (14, 2, 7),
            (15, 3, 5),
            (21, 3, 7),
            (35, 5, 7),
        ]);

        assert_eq!(HashSet::from(factors), factorize(primes));
    }
}
