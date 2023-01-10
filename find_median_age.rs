use std::{cmp::Ordering, collections::BinaryHeap};
#[derive(Eq, PartialEq)]
struct Reverse {
    age: i32,
}

impl PartialOrd for Reverse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.age.partial_cmp(&self.age)
    }
}

impl Ord for Reverse {
    fn cmp(&self, other: &Reverse) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct MedianOfAges {
    large_list: BinaryHeap<Reverse>, //containing second half of numbers
    small_list: BinaryHeap<i32>,   //containing first half of numbers
}


impl MedianOfAges {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianOfAges {
            large_list: BinaryHeap::new(),
            small_list: BinaryHeap::new(),
        }
    }

    fn insert_age(&mut self, num: i32) {
        if self.large_list.is_empty() || num > self.large_list.peek().unwrap().age {
            self.large_list.push(Reverse { age: num });
        } else {
            self.small_list.push(num);
        }
        // either both the heaps will have equal number of elements or min-heap will have one 
        // more element than the max-heap
        if self.large_list.len() > self.small_list.len() + 1 {
            self.small_list.push(self.large_list.pop().unwrap().age);
        } else if self.large_list.len() < self.small_list.len() {
            self.large_list.push(Reverse {age: self.small_list.pop().unwrap()});
        }
    }

    fn find_median(&self) -> f64 {
        if self.large_list.len() == self.small_list.len() {
            // we have even number of elements, take the average of middle two elements
            return (self.large_list.peek().unwrap().age as f64 + *self.small_list.peek().unwrap() as f64) / 2.0;
        } 
            //because min-heap will have one more element than the max-heap
            self.large_list.peek().unwrap().age as f64
        
    }
}


fn main()
{
    // Driver code
  let mut obj = MedianOfAges::new();
        obj.insert_age(22);
        obj.insert_age(35);
        println!("{}{:}","The recommended content will be for ages under: ",obj.find_median());     
        obj.insert_age(30);
        println!("{}{:}","The recommended content will be for ages under: ",obj.find_median());
        obj.insert_age(25);
        println!("{}{:}","The recommended content will be for ages under: ",obj.find_median());
       
}
