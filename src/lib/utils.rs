pub fn remove_or_zero(vec: &mut Vec<u8>, index: usize) -> u8 {
    if vec.len() != 0 {
        vec.remove(index)
    } else {
        0
    }
}

pub fn remove_or_false(vec: &mut Vec<bool>, index: usize) -> bool {
    if vec.len() != 0 {
        vec.remove(index)
    } else {
        false
    }
}

// The point of this si to convert all bits (`0` or `1` u8) to bytes (full u8)
pub fn bits_to_bytes(mut bin: Vec<u8>) -> Vec<u8> {
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

pub fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    let mut final_bin = Vec::new();

    for byte in bytes {
        let bin = format!("{:08b}", byte);

        for bit in bin.chars() {
            final_bin.push(bit == '1');
        }
    }

    final_bin
}
