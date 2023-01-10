fn calculate_minimum_servers(workload: Vec<i32>, rem: i32,mut counter:&mut Vec<i32>)-> i32{
      if rem < 0
          {return -1;}
      if rem == 0
          {return 0;}
      if counter[rem as usize - 1] != std::i32::MAX
          {return counter[rem as usize - 1];}
      let mut min = std::i32::MAX;

      for server in workload.clone().into_iter(){
          let result = calculate_minimum_servers(workload.to_vec(), rem - server ,&mut counter);
          if result >= 0 && result < min
            {  min = 1 + result;}
      }

      if min !=  std::i32::MAX
       { counter[rem as usize - 1] =  min;}
      else  
      {  counter[rem as usize - 1] = -1;}

      return counter[rem as usize - 1];
    }

fn find_minimum_servers(workload: Vec<i32>, demand: i32)->i32{
  if demand < 1
      {return 0;}
  let mut d: Vec<i32> =Vec::new();
  for _x in 0..demand{
    d.push(std::i32::MAX);
  }
  return calculate_minimum_servers(workload.to_vec(), demand ,&mut d);
}



fn main(){
  let workload: Vec<i32> = vec![2,3,4];
  let demand = 8;
  println!("{:}",find_minimum_servers(workload, demand));
}
