use serde::{Deserialize, Serialize};

use crate::lib::{bytes_to_bits, remove_or_false};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pair {
    pub value: u32,
    pub contents: Box<PairType>,
    pub child_chars: Vec<char>,
}

impl Pair {
    pub fn new(value: u32, contents: PairType, child_chars: Vec<char>) -> Self {
        Pair {
            value,
            contents: Box::new(contents),
            child_chars,
        }
    }

    pub fn from_char(c: char, v: u32) -> Self {
        Pair {
            value: v,
            contents: Box::new(PairType::Char(c)),
            child_chars: vec![c],
        }
    }

    pub fn to_deflated_string(&self) -> String {
        let mut compressed = String::new();

        compressed += &serde_json::to_string(self).unwrap();
        compressed += &String::from_utf8(vec![0x00]).unwrap();

        compressed
    }

    pub fn children_to_char_vec(&self) -> Vec<char> {
        let mut chars: Vec<char> = Vec::new();

        match &*self.contents {
            PairType::Char(c) => chars.push(*c),
            PairType::Pair((a, b)) => {
                let mut a_chars = a.children_to_char_vec();
                let mut b_chars = b.children_to_char_vec();

                chars.append(&mut a_chars);
                chars.append(&mut b_chars);
            }
        }

        chars
    }

    pub fn contains(&self, c: &char) -> bool {
        self.child_chars.contains(c)
    }

    pub fn encode_chars(&self, chars: &[char]) -> Vec<u8> {
        let mut bin = Vec::new();

        for c in chars {
            bin.append(&mut self.encode_char(c));
        }

        bin
    }

    pub fn encode_char(&self, c: &char) -> Vec<u8> {
        let mut bin = Vec::new();

        match &*self.contents {
            PairType::Char(_) => {
                panic!("Character encoder shouldn't have reached this far into the huffman tree")
            }
            PairType::Pair((a, b)) => {
                if a.contains(c) {
                    match &*a.contents {
                        PairType::Char(character) => {
                            if c != character {
                                panic!("Characters are not the same. The compression tree appears to have corrupted.");
                            }
                            bin.push(1)
                        }
                        PairType::Pair(_) => {
                            bin.push(1);
                            bin.append(&mut a.encode_char(&c.clone()));
                        }
                    }
                } else if b.contains(c) {
                    match &*b.contents {
                        PairType::Char(character) => {
                            if c != character {
                                panic!("Characters are not the same. The compression tree appears to have corrupted.");
                            }
                            bin.push(0);
                        }
                        PairType::Pair(_) => {
                            bin.push(0);
                            bin.append(&mut b.encode_char(&c.clone()));
                        }
                    }
                } else {
                    panic!(format!("The character '{}' is not in the huffman tree", c));
                }
            }
        }

        bin
    }

    pub fn decode_chars(&self, chars: &[u8]) -> String {
        let mut final_string = String::new();

        let mut bin = bytes_to_bits(chars);

        while bin.len() != 0 {
            final_string.push(self.decode_char(&mut bin));
        }

        final_string
    }

    pub fn decode_char(&self, c: &mut Vec<bool>) -> char {
        return match &*self.contents {
            PairType::Char(c) => c.clone(),
            PairType::Pair((a, b)) => {
                let bit = remove_or_false(c, 0);

                return match bit {
                    true => a.decode_char(c),
                    false => b.decode_char(c),
                };
            }
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PairType {
    Char(char),
    Pair((Pair, Pair)),
}
