use std::collections::HashMap;
fn main(){
    let mut intergers = vec![ 8, 2, 3, 6, 2, 7, 10, 3, ];

    //Median
    let length = intergers.len();
    for _ in 0..(length-1){
        for j in 0..(length-1){
            if &intergers[j] > &intergers[j+1] {
                intergers.swap(j, j + 1);
            }
        }
    }
    let middle = length / 2;
    let median = if length % 2 == 1 {intergers[middle] as f64 } else {(&intergers[middle - 1] + &intergers[middle]) as f64 / 2.0 };
    println!("The Median of the Intergers is {median}");

    // Mode
    let mut count_map = HashMap::new();
    for value in &intergers {
        let count = count_map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_count_value = &intergers[0];
    for (key, value) in &count_map {
        if max_count <= *value {
            max_count_value = *key;
            max_count = *value;
        }
    }
    println!("The Mode of the Vector List is {}", max_count_value);
}