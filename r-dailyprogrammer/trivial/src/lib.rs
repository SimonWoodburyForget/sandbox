pub mod codewars;
pub mod date_manipulation;
pub mod sorting;

use std::fmt::Write;

fn fizzbuzz() -> String {
    fizzbuzz_folder_to_string()
}

#[inline(always)]
pub fn fizzbuzz_folder_write() -> String {
    (1..101).fold(String::new(), |mut output, x| {
        let fizz = if x % 3 == 0 { "fizz" } else { "" };
        let buzz = if x % 5 == 0 { "buzz" } else { "" };
        if fizz.len() + buzz.len() != 0 {
            output + fizz + buzz + "\n"
        } else {
            write!(&mut output, "{}", x).unwrap();
            output + "\n"
        }
    })
}

#[inline(always)]
pub fn fizzbuzz_folder_to_string() -> String {
    (1..101).fold(String::new(), |output, x| {
        let fizz = if x % 3 == 0 { "fizz" } else { "" };
        let buzz = if x % 5 == 0 { "buzz" } else { "" };
        if fizz.len() + buzz.len() != 0 {
            output + fizz + buzz + "\n"
        } else {
            output + &x.to_string() + "\n"
        }
    })
}

fn main() {
    println!("{}", fizzbuzz());
}

#[test]
fn test_fizzbuzz() {
    println!("{}", fizzbuzz());
    assert!(!fizzbuzz().contains("15"));
    assert!(!fizzbuzz().contains("100"));
    assert!(!fizzbuzz().contains("50"));
    assert!(!fizzbuzz().contains("90"));
    assert!(fizzbuzz().contains("89\nfizzbuzz\n"));
    assert!(&fizzbuzz().starts_with("1\n2\n"));
    assert!(&fizzbuzz().ends_with("fizz\nbuzz\n"));
}
