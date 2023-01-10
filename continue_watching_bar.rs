use std::collections::HashMap;
pub struct FreqStack {
  pub freq: HashMap<String,i32>,
  pub group_map: HashMap<i32, Vec<String>>,
  pub max_freq: i32,
}

impl FreqStack {
  pub fn new() ->Self { 
    FreqStack {
      freq: HashMap::new(),
      group_map: HashMap::new(),
      max_freq:0,
    }
  }

  pub fn push(&mut self, show_name: String) {
        if self.freq.contains_key(&show_name)
          {  *self.freq.get_mut(&show_name).unwrap() +=1;}
        else
          {  self.freq.insert(show_name.to_string(),1);}
        
        if self.freq.get(&show_name).unwrap() > &self.max_freq
          {  self.max_freq = *self.freq.get(&show_name).unwrap();}

        if self.group_map.contains_key(&self.freq.get(&show_name).unwrap())
          {  self.group_map.get_mut(&self.freq.get(&show_name).unwrap()).unwrap().push(show_name);     } 
        else{
            self.group_map.insert(*self.freq.get(&show_name).unwrap(),vec![]);
            self.group_map.get_mut(&self.freq.get(&show_name).unwrap()).unwrap().push(show_name);
        }       
    }

  pub fn pop(&mut self)-> String {  
      if self.group_map.contains_key(&self.max_freq) {
          let x: String = self.group_map.get_mut(&self.max_freq).unwrap().pop().unwrap();
          *self.freq.get_mut(&x).unwrap()-=1;
          if self.group_map.get(&self.max_freq).unwrap().len() == 0
          {self.max_freq-=1;}
          return x;
      }
      return "".to_string();
  }

}

fn main(){
    
    let mut obj: FreqStack = FreqStack::new();
    obj.push("Queen's Gambit".to_string());
    obj.push("Teen Wolf".to_string());
    obj.push("Queen's Gambit".to_string());
    obj.push("Teen Wolf".to_string());
    obj.push("Bigderton".to_string());
    obj.push("Queen's Gambit".to_string());
    for _i in 0..7{
        println!("...User navigates to the browser...");
        println!("{}{}","Continue Watching :", obj.pop());
        println!();
    }
}
