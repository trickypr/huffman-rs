use std::{fs, fs::File, io::prelude::*, io::Error, iter::FromIterator, path::Path};

mod lib;

use lib::{
    compression::{compress, generate_tree},
    decompression::decompress,
    utils::bits_to_bytes,
};

fn main() -> Result<(), Error> {
    // Read the example text file from string
    // Todo: Allow you to specify file from commandline parameters
    let file = fs::read_to_string("./testText.txt")?;

    // Create a writable file
    let mut new_file = File::create("test.hc")?;

    // Compress file
    let (_, compressed) = compress(&file);

    // Add the compressed data to the file
    new_file.write_all(&compressed)?;

    // Open compressed file
    let mut file = File::open(&Path::new("test.hc"))?;

    // Load it into contents
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Decompress the contents and print it to the console
    println!("{}", decompress(contents));

    Ok(())
}
