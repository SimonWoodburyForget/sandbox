use necklace_matching::*;

fn main() {
    analyze(include_str!("../inputs/enable1.txt"));

    let v: Vec<&str> = include_str!("../inputs/enable1.txt")
        .trim()
        .split("\n")
        .collect();
    println!("{:?}", slicer::find_the_four(&v));
}
