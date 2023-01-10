use std::collections::HashMap;
fn smallest_sequence(list_a: Vec<String>, list_b: Vec<String>)-> Vec<String> {
    // Returns an empty list if one or more list is empty.
    if list_a.len() == 0 || list_b.len() == 0 {
      return vec![];
    }

    // We will use this dictionary to keep a count of all unique topics in list b.
    let mut dictlist_b: HashMap<String,i32> = HashMap::new();
    for i in 0..list_b.len()  {
      let count ;
      if dictlist_b.contains_key(&list_b[i])
      {
        count = *dictlist_b.get(&list_b[i]).unwrap();
        *dictlist_b.get_mut(&list_b[i]).unwrap() = count + 1;
      }
      else{
        count = 0;
        dictlist_b.entry(list_b[i].to_string()).or_insert( count + 1);
      }
      
    }

    // This will hold the count of number of unique topics in list b.
    let count_unique_b = dictlist_b.len();

    // unique_characters keeps a count of the number of unique characters of list_b which are present in the current with its required frequency.
    let mut unique_characters = 0;

    // This dictionary holds the count of all the unique topics in the current window.
    let mut unique_topics: HashMap<String,i32> = HashMap::new();

    // Checks and appends a topic along with its index from list A in sifted list, if the topic is present in list B.
    let mut sifted_list_a: Vec<(i32, String)> = Vec::new();
    for i in 0..list_a.len() {
      let strr = list_a[i].to_string();
      if dictlist_b.contains_key(&strr) {
        sifted_list_a.push((i as i32,strr));
      }
    }

    // A tuple that is used to store the window length, left, right
    let mut tuple_wllr: Vec<i32> = vec![std::i32::MAX, 0, 0];

    // Left and right pointers
    let mut left = 0;
    let mut right = 0;

    // Look for the topics only in the filtered list instead of entire list.
    while right < sifted_list_a.len() {
      // Add one topic from the right to the window
      let mut topic = sifted_list_a[right].1.to_string();

      let count ;
      if unique_topics.contains_key(&topic)
      {
        count = *unique_topics.get(&topic).unwrap();
        *unique_topics.get_mut(&topic).unwrap() = count+1;
      }
      else{
        count = 0;
        unique_topics.entry(topic.clone()).or_insert(count+1);
      }

      // Checking the frequency of the currently added topic.
      // If it is equal to the desired count in our list_b then we are incrementing the value of unique_characters.
      if dictlist_b.contains_key(&topic) &&  unique_topics.get(&topic) == dictlist_b.get(&topic) {
        unique_characters+=1;
      }

      // If the current window has all the topics in desired frequencies i.e. it is present in the window
      while left <= right && unique_characters == count_unique_b {
        topic = sifted_list_a[left].1.to_string();
        
        // Save the smallest window until now.
        let end_sequence:i32 = sifted_list_a[right].0;
        let start_sequence:i32 = sifted_list_a[left].0;
        if  end_sequence - start_sequence + 1 < tuple_wllr[0] {
          tuple_wllr[0] = end_sequence - start_sequence + 1;
          tuple_wllr[1] = start_sequence;
          tuple_wllr[2] = end_sequence;
        }

        // The character at the position pointed by the `left` pointer is no longer a part of the window.
        *unique_topics.get_mut(&topic).unwrap() -= 1;
        if dictlist_b.contains_key(&topic) &&  unique_topics.get(&topic).unwrap() <  dictlist_b.get(&topic).unwrap() {
          unique_characters-=1;
        }

        // Moving the left pointer ahead.
        left+=1;
      }

      // Moving the right pointer ahead.
      right+=1;
    }

    if tuple_wllr[0] == -1 {
      return vec![];
    } else {
      let mut res: Vec<String> = Vec::new();
      for i in tuple_wllr[1]..tuple_wllr[2] + 1 {
        res.push(list_a[i as usize].to_string() );
      }
      return res;
    }
  }

fn main() {
  // Driver code
  let a: Vec<String> = vec!["corona","petrol","corona","corona","climate","petrol","corona","petrol"].into_iter().map(String::from).collect();
  let b: Vec<String> = vec!["corona","petrol","climate"].into_iter().map(String::from).collect();

  let output = smallest_sequence(a, b);
  println!("{:?}",output);
}
