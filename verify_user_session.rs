fn verify_session( push_op: Vec<i32>,  pop_op: Vec<i32>) -> bool {
    let m = push_op.len();
    let n = pop_op.len();

    if m != n
    {
        return false;
    }
    
    let mut stack = Vec::new();
    let mut i = 0;

    for pid in push_op {
        stack.push(pid);
        while !stack.is_empty() && stack.last() == pop_op.get(i) {
            stack.pop();
            i=i+1;
        }
    }

    return stack.is_empty();
}

fn main(){
    // Driver code
    let push_op = vec![1,2,3,4,5];
    let pop_op = vec![5,4,3,2,1];

    if verify_session(push_op, pop_op)
    {
        println!("Session Successfull!");
    }
    else
    {
        println!("Session Faulty!");
    }
}
