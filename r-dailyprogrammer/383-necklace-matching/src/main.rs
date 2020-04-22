use necklace_matching::*;

fn main() {
    let prime = Primes::sieve_erato(2500);
    let count = prime.necklaces_big(1024, 512);
    // let count = prime.necklaces_big(1024, 1024 * 2);
    println!("{}", count);
    // analyze(include_str!("../inputs/enable1.txt"));

    // let v: Vec<&str> = include_str!("../inputs/enable1.txt")
    //     .trim()
    //     .split("\n")
    //     .collect();
    // println!("{:?}", find_the_four_counters(&v));
}
