fn max_profit(a: Vec<i32>) ->i32 {
    if a.len() < 1 {
      return 0;
    }

    let mut curr_max = a[0];
    let mut global_max = a[0];
    for i in 0..a.len() {
      if curr_max < 0
      {
        curr_max = a[i];
      }
      else{
        curr_max += a[i];
      }

      if global_max < curr_max
      {
        global_max = curr_max;
      }
    }

    return global_max;
  }
  
fn main() {
    // Driver code
    
    let stocks: Vec<i32> = vec![-4, 2, -5, 1, 2, 3, 6, -5, 1];
    println!("{}{:}{}","Maximum Profit: " , max_profit(stocks) , "%" );
}
