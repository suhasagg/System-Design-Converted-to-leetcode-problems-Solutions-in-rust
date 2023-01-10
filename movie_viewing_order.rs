fn backtrack( n: i32,  mut movies_list: &mut Vec<String>, mut output:&mut Vec<Vec<String>>, first: i32) {
    
    // If all integers of given array `Movies` are used and 
    // and Backtracking is performed add the permutations to Output array.
    if first == n
      {output.push(movies_list.to_vec());}
    
    // Perform Backtracking for the Size of a given array.
    for i  in first as usize..n as usize {
      
      // Swap: In the current permutaion place i-th integer first.
      let temp = movies_list[first as usize].to_string();
      movies_list[first as usize] = movies_list[i].to_string();
      movies_list[i] = temp.to_string();
      
      // Complete permuations using the next integers.
      backtrack(n,&mut movies_list,&mut output, first + 1);
      
      // Swap and Backtrack
      let temp2 = movies_list[first as usize].to_string();
      movies_list[first as usize] = movies_list[i].to_string();
      movies_list[i] = temp2.to_string();
    }
  }

fn generate_permutations(movies: Vec<String>)-> Vec<Vec<String>> {
    // init output list
    let mut output: Vec<Vec<String>> = Vec::new();

    // convert movies into list since the output is a list of lists
    let mut movies_list: Vec<String> = Vec::new();
    for num in movies.clone().into_iter(){
      movies_list.push(num);
    }

    let n = movies.len() as i32;
    backtrack(n,&mut movies_list,&mut output, 0);
    return output;
  }

fn main(){

    // Example #1
    let mut input: Vec<String> = vec!["Frozen","Dune","Coco"].into_iter().map(String::from).collect();
    let mut output = generate_permutations(input);
    println!("Output 1: ");
    println!("{:?}",output);

    // Example #2
    input = vec!["Frozen","Dune","Coco","Melificient"].into_iter().map(String::from).collect();
    output = generate_permutations(input);
    println!("Output 2: ");
    println!("{:?}",output);

  //   // Example #3
    input = vec!["Dune","Coco"].into_iter().map(String::from).collect();
    output = generate_permutations(input);
    println!("Output 3: ");
    println!("{:?}",output);

}
