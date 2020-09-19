use serde::{Deserialize, Serialize};

use super::super::{
    compression::{Pair, PairType},
    utils::{bytes_to_bits, remove_or_false},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeflatedPair {
    pub v: usize,
    pub c: Box<DeflatedPairType>,
}

impl DeflatedPair {
    pub fn from_pair(pair: &Pair) -> Self {
        let contents;

        match &*pair.contents {
            PairType::Char(c) => contents = DeflatedPairType::C(c.clone()),
            PairType::Pair((a, b)) => {
                contents =
                    DeflatedPairType::P((DeflatedPair::from_pair(a), DeflatedPair::from_pair(b)))
            }
        }

        DeflatedPair {
            v: pair.value,
            c: Box::new(contents),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
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
        return match &*self.c {
            DeflatedPairType::C(c) => c.clone(),
            DeflatedPairType::P((a, b)) => {
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
pub enum DeflatedPairType {
    P((DeflatedPair, DeflatedPair)),
    C(char),
}
