use std::collections::HashMap;
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Node {
  children: HashMap<char, Node>,
  is_word: bool,
}

impl Node {
  pub fn new() ->Self { 
    Node {
      is_word: false,
      children: HashMap::new()
    }
  }
}
#[derive(Default,Debug, PartialEq, Eq, Clone)]
pub struct WordDictionary{
  pub root: Node
}

impl WordDictionary {
  pub fn new() ->Self { 
    Self::default()
  }
  fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for w in word.chars() {
            current = current.children.entry(w).or_insert(Node::new());
        }

        if !current.is_word {
            current.is_word = true;
        }
    }

    pub fn search(&mut self, word: String) -> bool {
        let mut current = &mut self.root;
        for w in word.chars() {
            if let Some(_) = current.children.get(&w) {
                current = current.children.entry(w).or_insert(Node::new());
            } else {
                return false;
            }
        }
        current.is_word
    }

    pub fn starts_with(&mut self, prefix: String) -> bool {
        let mut current = &mut self.root;
        for w in prefix.chars() {
            if let Some(_) = current.children.get(&w) {
                current = current.children.entry(w).or_insert(Node::new());
            } else {
                return false;
            }
        }
        true
    }

}


fn main()
{
  let keys: Vec<String> = vec!["the", "a", "there", "answer", "any",
    "by", "bye", "their", "abc"].into_iter().map(String::from).collect();
    println!("{}{:?}","Keys to insert: " ,keys);
  
    let mut d = WordDictionary::new();
    for i in 0..keys.len(){
        d.insert(keys[i].to_string());
    }
   println!("{}{:}","Searching 'there' in the dictionary results: " , d.search("there".to_string()) );

}
