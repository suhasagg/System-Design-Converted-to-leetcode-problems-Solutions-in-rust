fn search(arr: Vec<i32>, start: i32, end: i32, key: i32) -> i32{  
    if start > end {
      return -1;
    }

    let mid = start + (end - start) / 2;

    if arr[mid as usize] == key {
        return mid as i32;
    }

    if arr[start as usize] <= arr[mid as usize] && key <= arr[mid as usize] && key >= arr[start as usize] {
      return search(arr, start as i32, mid - 1 as i32, key);
    }

    else if arr[mid as usize] <= arr[end as usize] && key >= arr[mid as usize] && key <= arr[end as usize] {
      return search(arr, mid + 1 as i32, end as i32, key);
    }

    else if arr[end as usize] <= arr[mid as usize] {
      return search(arr, mid + 1 as i32, end as i32, key);
    }

    else if arr[start as usize] >= arr[mid as usize] {
      return search(arr, start, mid - 1 as i32, key);
    }

    return -1;
}

fn find_story_id(arr: &Vec<i32>, key: i32) -> i32 {
    return search(arr.to_vec(), 0, (arr.len() - 1) as i32, key);
}

fn main(){
    let v1 = vec![6, 7, 1, 2, 3, 4, 5];
    println!("{}{:}","Key(3) found at: ",find_story_id(&v1, 3).to_string());
    println!("{}{:}","Key(6) found at: ",find_story_id(&v1, 6).to_string());
    let v2 = vec![4, 5, 6, 1, 2, 3];
    println!("{}{:}","Key(3) found at: ",find_story_id(&v2, 3).to_string());
    println!("{}{:}","Key(6) found at: ",find_story_id(&v2, 6).to_string());
}
