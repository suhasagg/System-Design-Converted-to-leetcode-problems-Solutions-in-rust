use std::collections::HashSet;
fn length_of_longest_busy_period(schedule:  Vec<i32>) -> i32{
    let mut longest_busy_period = 0;
    let  slot_set: HashSet<_> = schedule.into_iter().collect();
    for slot in &slot_set{
        if !slot_set.contains(&(slot - 1)) {
            let mut current_slot = slot.to_owned();
            let mut current_consecutive_slot = 1;
            while slot_set.contains(&(current_slot + 1)) {
                current_slot=current_slot+1;
                current_consecutive_slot+=1;
            }
            if current_consecutive_slot > longest_busy_period
            {
                longest_busy_period = current_consecutive_slot;
            }
        }
    }

    return longest_busy_period;
}


fn main() {
    let schedule: Vec<i32> = vec![3, 1, 8, 5, 2, 12,10, 4, 8, 9];
    println!("{:}",length_of_longest_busy_period(schedule)) ;
}
