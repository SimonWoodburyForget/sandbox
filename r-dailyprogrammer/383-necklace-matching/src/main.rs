use necklace_matching::*;

fn main() {
    analyze(include_str!("../inputs/enable1.txt"));

    simple::find_the_four(
        include_str!("../inputs/enable1.txt")
            .trim()
            .split("\n")
            .collect(),
    );
}
