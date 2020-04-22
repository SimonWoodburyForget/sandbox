use std::{error::Error, fmt, fs::File, io::prelude::*};

#[derive(Debug)]
struct ParsingError;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CSV parsing error!")
    }
}

impl Error for ParsingError {}

pub fn read_batches_original(paths: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    paths
        .iter()
        .map(|file| {
            let mut s = String::new();
            File::open(file)?.read_to_string(&mut s)?;

            let len = s.split(",").take(5).count();

            if len != 5 {
                Err(ParsingError.into())
            } else {
                Ok(s)
            }
        })
        .collect()

    // for file in paths {
    //     let file = File::open(file).unwrap();
    //     let reader = BufReader::new(file);

    //     for line in reader.lines() {
    //         let line = line.unwrap();

    //         let splitbycomma = line
    //             .split(',')
    //             .map(|s| &s[1..(s.len() - 1)])
    //             .collect::<Vec<&str>>();

    //         if splitbycomma.len() != 5 {
    //             continue;
    //         }

    //         let bid = splitbycomma[0];
    //         let name = splitbycomma[2];
    //         let batch = Batch {
    //             batch_id: bid.to_owned(),
    //             batch_type: name.to_owned(),
    //         };
    //         batches.push(batch);
    //     }
}

fn main() {
    println!("Hello, world!");
}
