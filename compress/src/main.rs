extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();
    // arguments name file and name compress file, first argument is to run a script
    if args.len() != 3 {
        eprint!("Usage: `source` `target`");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let input_file = File::open(input_path).unwrap();
    let output_file = File::create(output_path).unwrap();

    let mut input = BufReader::new(input_file);

    let output_metadata = output_file.metadata().unwrap();

    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output_metadata.len());
    println!("Elapsed: {:?}", start.elapsed());
}
