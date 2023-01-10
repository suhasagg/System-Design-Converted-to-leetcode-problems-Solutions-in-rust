use std::collections::HashMap;
extern crate priority_queue; 
use priority_queue::PriorityQueue;
use priority_queue::DoublePriorityQueue;

fn median_sliding_window( nums: Vec<i32>, k: i32)-> Vec<f64>
{
    let mut medians: Vec<f64> = Vec::new();
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    let mut small_list = PriorityQueue::new();  // max heap
    let mut large_list = DoublePriorityQueue::new();   // min heap

    let mut i = 0;      // index of current incoming element being processed

    // initialize the heaps
    while i < k{
        small_list.push(nums[i as usize],nums[i as usize]);
        i +=1;
    }
    for _j in 0..k / 2 {
        let small = small_list.pop().unwrap().0;
        large_list.push(small,small);
    }
    
    loop {
        // get median of current window
        if (k & 1) == 1{
          medians.push(*small_list.peek().unwrap().0 as f64);
        }
        else{
          let s = (*small_list.peek().unwrap().0 as f64 + *large_list.peek_min().unwrap().0 as f64) * 0.5;
          medians.push(s);
        }

        if i >= nums.len() as i32
            {break;   }                       // break if all elements processed

        let out_num = nums[(i - k)as usize];          // outgoing element
        let in_num = nums[i as usize];          // incoming element
        i+=1;
        let mut balance: i32 =0;                    // balance factor

        // number `out_num` exits window
        if out_num <= *small_list.peek().unwrap().0{
          balance += -1;
        }
        else{
          balance += 1;
        }

        if !hashmap.contains_key(&out_num){
            hashmap.entry(out_num).or_insert(1);
        }
        else{
          *hashmap.get_mut(&out_num).unwrap() +=1;
        }
        // number `in_num` enters window
        if !small_list.is_empty() && in_num <= *small_list.peek().unwrap().0 {
            balance+=1;
            small_list.push(in_num.clone(),in_num.clone());
        }
        else {
            balance-=1;
            large_list.push(in_num.clone(),in_num.clone());
        }

        // re-balance heaps
        if balance < 0 {                  // `small_list` needs more valid elements
            small_list.push(*large_list.peek_min().unwrap().0,*large_list.peek_min().unwrap().0);
            large_list.pop_min();
            balance+=1;
        }
        if balance > 0 {                  // `hi` needs more valid elements
            large_list.push(*small_list.peek().unwrap().0,*small_list.peek().unwrap().0);
            small_list.pop();
            
        }
        // remove invalid numbers that should be discarded from heap peeks
        if hashmap.contains_key(small_list.peek().unwrap().0) {
          while !hashmap.get(small_list.peek().unwrap().0).is_none() && hashmap.get(small_list.peek().unwrap().0).unwrap() > &0{
              *hashmap.get_mut(small_list.peek().unwrap().0).unwrap()-=1;
              small_list.pop();
          }
        }
        if !large_list.is_empty(){
          if hashmap.contains_key(large_list.peek_min().unwrap().0){
            while !large_list.is_empty() && *hashmap.get(large_list.peek_min().unwrap().0).unwrap() >0 {
                *hashmap.get_mut(large_list.peek_min().unwrap().0).unwrap()-=1;
                large_list.pop_min();
                if !hashmap.contains_key(large_list.peek_min().unwrap().0)
                {
                  break;
                }
            }
          }
        }
    }

    return medians;
}

fn main() {
    // Driver code

    println!("Example - 1" );
    let arr: Vec<i32> = vec![1,3,-1,-3,5,3,6,7];
    let mut k = 3;
    println!("{}{:?}{}{:}","Input: array = " ,arr ,", k = " , k );
    let output = median_sliding_window(arr, k);
    println!("{}{:?}","Output: Medians = " ,output);

    println!("Example - 2");
    let arr2 = vec![1,2];
    k = 1;
    println!("{}{:?}{}{:}","Input: array = " ,arr2 ,", k = " , k );
    let output2 = median_sliding_window(arr2, k);
    println!("{}{:?}","Output: Medians = " ,output2);
}
