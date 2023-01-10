fn dfs (friends: &Vec<Vec<bool>>, n: i32, mut visited: &mut Vec<bool>, v: i32) {
  for i in 0..n  {

      // A user is in the friend circle if he/she is friends with the user represented by
      // user index and if he/she is not already in a friend circle
      if friends[v as usize][i as usize] == true && !visited[i as usize] && i != v {
          visited[i as usize] = true;
          dfs(&friends, n, &mut visited, i as i32) ;
      }
  }
}

fn friend_circles(friends: Vec<Vec<bool>>, n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut num_circles = 0;     //Number of friend circles
    
    //Keep track of whether a user is already in a friend circle
    
    let mut visited: Vec<bool> = vec![false;n as usize];
    //Start with the first user and recursively find all other users in his/her
    //friend circle. Then, do the same thing for the next user that is not already
    //in a friend circle. Repeat until all users are in a friend circle. 
     for i in 0..n as usize {
        if !visited[i] {
            visited[i] = true;
            dfs(&friends, n, &mut visited, i as i32); //Recursive step to find all friends
            num_circles = num_circles + 1;
        }
    }
     return num_circles;
}
fn main()
{
  let n = 4;
  let friends: Vec<Vec<bool>> =vec![vec![true,true,false,false],
      vec![true,true,true,false],
      vec![false,true,true,false],
      vec![false,false,false,true]];
      let num = friend_circles(friends,n);
      println!("{}{:}","Number of friends circles: ",num);
}
