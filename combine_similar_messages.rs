use std::collections::HashMap;
fn combine_messages(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut message_group: HashMap<Vec<u8>, Vec<String>> = HashMap::with_capacity(strings.len());

    for s in strings {
        let group = message_group.entry(generate_key(s.as_bytes())).or_insert(vec![]);
        group.push(s.clone());
    }

    message_group.into_iter().map(|(_, v)| v).collect()
}

fn generate_key(chars: &[u8]) -> Vec<u8> {
    let mut key = vec![0];

    for i in 1..chars.len() {
        key.push((chars[i] + 26 - chars[i - 1]) % 26);
    }

    key
}

fn main() {
    // Driver code
    
   let messages: Vec<String> = vec!["lmn", "mno", "azb", "bac", "yza", "bdfg"].into_iter().map(String::from).collect();
    let groups = combine_messages(messages);

    println!( "The Grouped Messages are:");
    println!("{:?}",groups);
}
