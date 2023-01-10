use std::collections::HashMap;
fn backtrack(index: i32,mut path:&mut Vec<String>, genres: Vec<String>,mut combinations:&mut Vec<String>) {
    // If the path is the same size() as genres, we have a complete combination
    if path.len() == genres.len() {
        let mut s ="".to_string();
        for i in 0..path.len()
        {
            s.push_str(&path[i]);
        }
        combinations.push(s);
        return; // backtrack
    }
    // Movies map
    let mut movies: HashMap<String, Vec<String>> = HashMap::new();
    movies.insert("Family".to_string(), vec!["Frozen", "Kung fu Panda", "Ice Age"].into_iter().map(String::from).collect());
    movies.insert("Action".to_string(), vec!["Iron Man", "Wonder Woman", "Avengers"].into_iter().map(String::from).collect());
    movies.insert("Fantasy".to_string(), vec!["Jumanji", "Lion King", "Tarzan"].into_iter().map(String::from).collect());
    movies.insert("Comedy".to_string(), vec!["Coco", "The Croods", "Vivi","Pets"].into_iter().map(String::from).collect());
    movies.insert("Horror".to_string(), vec!["Oculus", "Sinister","Insidious","Annebelle"].into_iter().map(String::from).collect());
    // Get the movies that the current digit maps to, and loop through them
    let possible_movies = movies.get(&genres[index as usize]).unwrap();
    for i in 0..possible_movies.len() {
        // Add the letter to our current path
        let con = format!("{}{}",possible_movies[i], ';');
        path.push(con);
        // Move on to the next digit
        backtrack(index + 1,&mut path, genres.to_vec(),&mut combinations);
        // backtrack by removing the letter before moving onto the next
        if path.len() > 0
            {path.pop();}
    }
}

fn letter_combinations( genres: Vec<String>)-> Vec<String> {
    // If the input is empty, immediately return an empty answer array
    let mut combinations: Vec<String> = Vec::new();
    if genres.len() == 0 {
        return combinations;
    }
    
    // Initiate backtracking with an empty path and starting index of 0
    let mut path: Vec<String> = Vec::new();
    backtrack(0,&mut path, genres,&mut combinations);
    return combinations;
}


fn main() {
    // Driver code
    // Example - 1
    let genres: Vec<String> = vec!["Action".to_string()];
    let mut res = letter_combinations(genres);
    println!("Output 1:");
    println!("{:?}",res);

    // Example - 2;
    let genres2: Vec<String> = vec!["Family", "Action"].into_iter().map(String::from).collect();
    res = letter_combinations(genres2);
    println!("Output 2:");
    println!("{:?}",res);

    // // Example - 3
    let genres3: Vec<String> = vec!["Horror", "Comedy"].into_iter().map(String::from).collect();
    res = letter_combinations(genres3);
    println!("Output 3:");
    println!("{:?}",res);

    // // // Example - 4
    let genres4: Vec<String> = vec!["Horror", "Fantasy", "Comedy", "Family"].into_iter().map(String::from).collect();
    res = letter_combinations(genres4);
    println!("Output 4:");
    println!("{:?}",res);
}
