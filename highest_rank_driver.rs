extern crate priority_queue; 
use priority_queue::DoublePriorityQueue;
fn kth_highest_rank( ranks: Vec<i32>, k:i32)->i32 {
    let mut minheap = DoublePriorityQueue::new();
    // Put first k ranks in the min heap
    for  i in 0..k{
      minheap.push(ranks[i as usize],ranks[i as usize]);
    }
    // Go through the remaining ranks of the array, if the rank from the array is greater than the
    // top (smallest) rank of the heap, remove the top rank from heap and add the rank from array
    for i in k..ranks.len() as i32 {
      if ranks[i as usize] > *minheap.peek_min().unwrap().0 {
        minheap.pop_min();
        minheap.push(ranks[i as usize],ranks[i as usize]);
      }
    }

    // The root of the heap has the Kth largest rank
    return *minheap.peek_min().unwrap().0;
}

fn main() {
    // Driver code

    let driver_id: Vec<i32> = vec![1, 5, 12, 2, 11, 9, 7, 30, 20];
    let k = 3; // Supplied by a hidden API

    println!("{}{:}{}","Driver with the rank " ,kth_highest_rank(driver_id, k) , " is selected!"); 
}
