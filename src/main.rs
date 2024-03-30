
use std::collections::HashMap;


fn max_key(vocab: &HashMap<String, i32>) -> Option<&String>
{
    vocab
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}


fn to_vocab(txt: String) -> HashMap<String, i32> {
    let mut vocab: HashMap<String, i32> = HashMap::new();

    for c in txt.chars(){
        vocab.entry(c.to_string()).and_modify(|counter| *counter += 1).or_insert(1);
    }
    return vocab;
}

fn main() {
    let vocab = to_vocab(String::from("blaaaa asdas ss"));
    let max_entry = max_key(&vocab);
    println!("{:?}", max_entry);

}
