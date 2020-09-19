use std::{collections::BTreeMap, iter::FromIterator};

use super::{super::utils::bits_to_bytes, Pair, PairType};

pub fn compress(text: &String) -> (Pair, Vec<u8>) {
    let tree = generate_tree(text);

    let mut tree_bytes = Vec::from(tree.deflate().to_string().as_bytes());
    let mut compressed =
        bits_to_bytes(tree.encode_chars(&Vec::from_iter(text.chars().into_iter())));

    let mut final_bytes = Vec::new();

    final_bytes.append(&mut tree_bytes);
    final_bytes.push(0);
    final_bytes.append(&mut compressed);

    (tree, final_bytes)
}

pub fn generate_tree(text: &String) -> Pair {
    // Check how common each character is in the file
    let mut chars = BTreeMap::new();
    text.chars().into_iter().for_each(|character| {
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

    chars.pop().unwrap()
}
