fn break_query( query: String,  dict: Vec<String>) ->bool{
    let mut dp: Vec<bool> = vec![false; query.len() + 1];
    dp[0] = true;
    for i in 0..query.len() {
        for j in dict.iter() {
          if dp[i]
          {
            let l=j.len();
            if i+l<=query.len() && &query[i..i + l].to_string() == j {
                dp[i+l] = true;
            }
          }
        }
    }
    return dp[query.len()];
}
fn main() {
    let mut query = String::from("vegancookbook");
    let dict: Vec<String> = vec!["i", "cream", "cook", "scream", "ice", "cat", "book", "icecream", "vegan"].into_iter()
        .map(String::from)
        .collect();
    println!("{:}",break_query(query, dict.clone()));

     query = String::from("veganicetea");
     println!("{:}",break_query(query, dict));

}
