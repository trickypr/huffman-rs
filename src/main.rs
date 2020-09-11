use ron::ser::to_string;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{collections::BTreeMap, fs, fs::File, io::prelude::*, io::Error, iter::FromIterator};

#[derive(Debug)]
enum PairType {
    Char(char),
    Pair((Pair, Pair)),
}

impl Serialize for PairType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &*self {
            PairType::Char(c) => serializer.serialize_newtype_variant("PairType", 0, "Char", &c),
            PairType::Pair(pair) => {
                serializer.serialize_newtype_variant("PairType", 1, "Pair", &pair)
            }
        }
    }
}

#[derive(Debug)]
struct Pair {
    value: u32,
    contents: Box<PairType>,
    child_chars: Vec<char>,
}

impl Pair {
    fn new(value: u32, contents: PairType, child_chars: Vec<char>) -> Self {
        Pair {
            value,
            contents: Box::new(contents),
            child_chars,
        }
    }

    fn from_char(c: char, v: u32) -> Self {
        Pair {
            value: v,
            contents: Box::new(PairType::Char(c)),
            child_chars: vec![c],
        }
    }

    fn to_deflated_string(&self) -> String {
        let mut compressed = String::new();

        compressed += &to_string(self).unwrap();
        compressed += &String::from_utf8(vec![0x00]).unwrap();

        compressed
    }

    fn children_to_char_vec(&self) -> Vec<char> {
        let mut chars: Vec<char> = Vec::new();

        match &*self.contents {
            PairType::Char(c) => chars.push(c.clone()),
            PairType::Pair((a, b)) => {
                let mut a_chars = a.children_to_char_vec();
                let mut b_chars = b.children_to_char_vec();

                chars.append(&mut a_chars);
                chars.append(&mut b_chars);
            }
        }

        chars
    }

    fn encode_char(&self, c: char) -> Vec<u8> {
        let mut bin = Vec::new();

        match &*self.contents {
            PairType::Char(c) => {
                panic!("Character encoder shouldn't have reached this far into the huffman tree")
            }
            PairType::Pair((a, b)) => {
                if a.child_chars.contains(&c) {
                    match &*a.contents {
                        PairType::Char(c) => bin.push(0),
                        PairType::Pair(_) => bin.append(&mut a.encode_char(c)),
                    }
                } else if b.child_chars.contains(&c) {
                    match &*b.contents {
                        PairType::Char(c) => bin.push(1),
                        PairType::Pair(_) => bin.append(&mut b.encode_char(c)),
                    }
                } else {
                    panic!(format!("The character '{}' is not in the huffman tree", c));
                }
            }
        }

        bin
    }
}

impl Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("pair", 3)?;

        state.serialize_field("value", &self.value)?;
        state.serialize_field("contents", &self.contents)?;
        state.serialize_field("child_chars", &self.child_chars)?;

        state.end()
    }
}

fn remove_or_zero(vec: &mut Vec<u8>, index: usize) -> u8 {
    if vec.len() != 0 {
        vec.remove(index)
    } else {
        0
    }
}

// The point of this si to convert all bits (`0` or `1` u8) to bytes (full u8)
fn bits_to_bytes(mut bin: Vec<u8>) -> Vec<u8> {
    let mut final_bin = Vec::new();

    while bin.len() != 0 {
        let a = remove_or_zero(&mut bin, 0);
        let b = remove_or_zero(&mut bin, 0);
        let c = remove_or_zero(&mut bin, 0);
        let d = remove_or_zero(&mut bin, 0);
        let e = remove_or_zero(&mut bin, 0);
        let f = remove_or_zero(&mut bin, 0);
        let g = remove_or_zero(&mut bin, 0);
        let h = remove_or_zero(&mut bin, 0);

        let bits = vec![a, b, c, d, e, f, g, h];

        final_bin.push(bits.iter().fold(0, |result, &bit| (result << 1) ^ bit));
    }

    final_bin
}

fn main() -> Result<(), Error> {
    // Read the example text file from string
    // Todo: Allow you to specify file from commandline parameters
    let file = fs::read_to_string("./testText.txt")?;

    // Check how common each character is in the file
    let mut chars = BTreeMap::new();
    file.chars().into_iter().for_each(|character| {
        if chars.contains_key(&character) {
            *chars.get_mut(&character).unwrap() += 1;
        } else {
            chars.insert(character, 1);
        }
    });

    // Convert to a Vec of `Pair`s
    let mut chars: Vec<Pair> = chars
        .into_iter()
        .map(|(character, value)| Pair::from_char(character, value))
        .collect();

    while chars.len() != 1 {
        // Sort the Vec to allow us to do do fun stuff with it
        chars.sort_by(|a, b| b.value.cmp(&a.value));

        let a = chars.pop().unwrap();
        let b = chars.pop().unwrap();

        let sum = a.value + b.value;

        let mut child_chars = Vec::new();

        child_chars.append(&mut a.children_to_char_vec());
        child_chars.append(&mut b.children_to_char_vec());

        chars.push(Pair::new(sum, PairType::Pair((a, b)), child_chars));
    }

    let tree = chars.pop().unwrap();

    let mut compressed = Vec::new();

    file.chars()
        .into_iter()
        .for_each(|character| compressed.append(&mut tree.encode_char(character)));

    compressed = bits_to_bytes(compressed);

    let mut new_file = File::create("test.z")?;
    new_file.write_all(tree.to_deflated_string().as_bytes())?;
    new_file.write_all(&compressed)?;

    // println!("{:?}", compressed);

    Ok(())
}
