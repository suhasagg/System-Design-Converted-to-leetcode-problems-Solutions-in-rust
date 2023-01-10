use std::collections::HashMap;
fn group_titles(strs: Vec<String>) -> Vec<Vec<String>>
{
    let mut res = HashMap::new();
    let mut count = vec![0; 26];

    for s in strs.into_iter() {
        for c in s.chars() {
            let index = (c as u32 - 'a' as u32) as usize;
            count[index] += 1;
        }
        
        let mut key = String::from("");
        
        for k in 0..26 {
            key=key+"#";
            key=key+&count[k].to_string();
        }
        res.entry(key).or_insert(Vec::new()).push(s);
        
        count=vec![0;26];
    }
    
    res.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    // Driver code
    let titles = vec!["duel".to_string(),"dule".into(),"speed".into(),"spede".into(),"deul".into(),"cars".into()];
    let output: Vec<Vec<String>> ;
    
   output = group_titles(titles);
    let query = String::from("spede");
    
   for o in output.into_iter() {
       if o.contains(&query)
       {
           println!("{:?}", o);
       }
       
   }
}
