use rand::prelude::*;
#[derive(Default)]
pub struct Uber{
  cum_sums: Vec<i32>,
}

impl Uber {
    fn new() -> Self {
        Default::default()
    }
    fn insert(&mut self, metrics: Vec<i32>)
    {
        let mut cum_sum=0;
        for i in 0..metrics.len() {
            cum_sum += metrics[i];
            self.cum_sums.push(cum_sum);
        }
    }

    fn pick_route(&self)->i32 {
      let mut rng = rand::thread_rng();
        let random:f64 =rng.gen(); 
        let sum:f64 = self.cum_sums[self.cum_sums.len() - 1] as f64;
        let target:f64 = random * sum;

        // Binary search to find the target
        let mut low = 0;
        let mut high = self.cum_sums.len();
        while low < high {
            let mid = low + (high - low) / 2;
            if target > self.cum_sums[mid] as f64{
                low = mid + 1;
            }
            else{
                high = mid;
            }
        }
        return low as i32;
    }
}

fn main() {
    // Driver code
    let metrics: Vec<i32> = vec![1, 2, 3];
    let mut uber= Uber::new();
    uber.insert(metrics);
    for _i in 0..10{
        println!("{}{:}","Randomly choose route " ,uber.pick_route());
    }
}
