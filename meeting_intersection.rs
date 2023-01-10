use std::cmp;
fn meetings_intersection( meetings_a: Vec<Vec<i32>> ,  meetings_b: Vec<Vec<i32>> ,  intersection: &mut Vec<Vec<i32>> ){
    let mut i = 0;
    let mut j = 0;
    while i < meetings_a.len() && j < meetings_b.len() {
        let start = cmp::max(meetings_a[i][0], meetings_b[j][0]);
        let end = cmp::min(meetings_a[i][1], meetings_b[j][1]);
        if start < end{
            intersection.push([start, end].to_vec());
        }
        if meetings_a[i][1] < meetings_b[j][1]{
            i+=1;
        }
        else{
            j+=1;
        }
    }
}
// Driver Code
fn main() {
    let meetings_a: Vec<Vec<i32>> = vec![vec![1, 3], vec![5, 6], vec![7, 9]];
    let meetings_b: Vec<Vec<i32>> = vec![vec![2, 3], vec![5, 7]];
    let mut intersection: Vec<Vec<i32>> =Vec::new() ;
    meetings_intersection(meetings_a, meetings_b,&mut intersection);
    println!("{:?}",intersection);
}
