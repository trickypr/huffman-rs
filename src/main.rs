use std::{fs, fs::File, io::prelude::*, io::Error, iter::FromIterator, path::Path};

mod decode;
mod encode;
mod lib;
mod pair;

use lib::utils::{bits_to_bytes, bytes_to_bits};

fn main() -> Result<(), Error> {
    // Read the example text file from string
    // Todo: Allow you to specify file from commandline parameters
    let file = fs::read_to_string("./testText.txt")?;

    let tree = encode::generate_tree(&file);
    let mut compressed = tree.encode_chars(&Vec::from_iter(file.chars().into_iter()));
    compressed = bits_to_bytes(compressed);

    let mut new_file = File::create("test.z")?;
    new_file.write_all(tree.to_deflated_string().as_bytes())?;
    new_file.write_all(&compressed)?;

    // Open compressed file
    let mut file = File::open(&Path::new("test.z"))?;

    // Load it into contents
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // println!("{:?}", contents);

    // Decompress the contents
    println!("{}", decode::decode(contents));
    // decode::decode(contents);

    Ok(())
}
