use super::DeflatedPair;

pub fn decompress(mut compressed: Vec<u8>) -> String {
    let mut current_char = compressed.remove(0);

    let mut tree_string = String::new();

    while current_char != 0 {
        tree_string.push_str(&format!("{}", current_char as char));
        current_char = compressed.remove(0);
    }

    let tree: DeflatedPair = serde_json::from_str(&tree_string).unwrap();

    tree.decode_chars(&compressed)
}
