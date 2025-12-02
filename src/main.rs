use std::collections::HashMap;

struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

fn main() {
    let input: String = String::from("this is a test for the huffman coding");
    let char_vec: Vec<char> = input.chars().filter(|&c| c != ' ').collect();
    let mut char_count: HashMap<char, i32> = HashMap::new();
    for c in char_vec.iter() {
        if char_count.contains_key(c)
            && let Some(freq) = char_count.get(c)
        {
            char_count.insert(*c, freq + 1);
        } else {
            char_count.insert(*c, 1);
        }
    }
    let mut sorted_freq: Vec<(&char, &i32)> = char_count.iter().collect();
    sorted_freq.sort_by(|a, b| a.1.cmp(&b.1));

    // Can Start Building the tree from here
    println!("{:?}", sorted_freq);
}
