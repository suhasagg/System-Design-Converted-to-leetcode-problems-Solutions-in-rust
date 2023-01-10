fn identify_titles(scores: Vec<i32>) -> bool
{
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len()-1 {
        if scores[i] > scores[i+1]
        {
            increasing = false;
        }
        if scores[i] < scores[i+1]
        {
            decreasing = false;
        }
    }
    return increasing || decreasing;
}

fn main() {
// Driver code
    let movie_ratings = vec![vec![1,2,2,3],
                        vec![4,5,6,3,4],
                        vec![8,8,7,6,5,4,4,1]
                    ];
    
    for movie_rating in movie_ratings {
       if identify_titles(movie_rating)
       {
           println!("Title Identified and Separated");
       }
       else
        {
            println!("Title Score Fluctuating");
        }
    }

}
