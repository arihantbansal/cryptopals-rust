pub fn main() {
    let first_input = "1c0111001f010100061a024b53535009181c";
    let first_input_as_bytes = hex::decode(first_input).unwrap();

    let second_input = "686974207468652062756c6c277320657965";
    let second_input_as_bytes = hex::decode(second_input).unwrap();

    let xor_output: Vec<u8> = first_input_as_bytes
        .iter()
        .zip(second_input_as_bytes.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    assert_eq!(
        hex::encode(xor_output),
        "746865206b696420646f6e277420706c6179"
    );
}
