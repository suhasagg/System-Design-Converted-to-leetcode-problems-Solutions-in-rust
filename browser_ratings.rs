fn main() {
  
    let mut stack = MaxStack::new();
    stack.push(5);
    stack.push(0);
    stack.push(2);
    stack.push(4);
    stack.push(6);
    stack.push(3);
    stack.push(10);

  print!("Maximum Rating: ");
  println!("{:}",stack.max_rating());

     stack.pop();
     println!("After clicking back button: ");
     print!("Maximum Rating: ");
     println!("{:}",stack.max_rating());
    
}
