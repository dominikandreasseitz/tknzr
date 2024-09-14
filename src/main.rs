use std::collections::HashMap;

fn max_key(map: &HashMap<(char, char), i32>) -> Option<&(char, char)> {
    map.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(k, _v)| k)
}

fn string_to_vocab(input_string: String) -> HashMap<(char, char), i32> {
    let characters: Vec<char> = input_string.chars().collect();
    let len_chars = characters.len();
    let mut vocab: HashMap<(char, char), i32> = HashMap::new();
    let it = characters[0..len_chars - 1]
        .iter()
        .zip(characters[1..len_chars].iter());
    for (c0, c1) in it {
        vocab
            .entry((*c0, *c1))
            .and_modify(|counter: &mut i32| *counter += 1)
            .or_insert(1);
    }
    vocab
}

fn main() {
    let mystring = String::from("hello world");
    let vocab = string_to_vocab(mystring);
    for (k, _v) in vocab.iter() {
        println!("{}, {}", k.0, k.1);
    }
    let _mxkey: Option<&(char, char)> = max_key(&vocab);
    println!("{:?}", _mxkey.unwrap());
}
