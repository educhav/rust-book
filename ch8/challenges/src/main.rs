use std::collections::HashMap;


#[derive(Debug)]
struct Stats {
    median: f32,
    mode: i32
}

fn median_mode(v: &Vec<i32>) -> Stats {
    let mut v_copy = v.clone();
    v_copy.sort();

    let median_index = v_copy.len() / 2;

    let mut median: f32 = v_copy[median_index] as f32;

    if v_copy.len() % 2 == 0 {
        median = (v_copy[median_index] + v_copy[median_index-1]) as f32 / 2.0;
    }

    let mut freqs = HashMap::new();
    let mut max_count = 0;
    let mut mode = v[0];

    for num in v {
        let count = freqs.entry(*num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = *num;
        }
    }

    Stats {
        median, mode
    }
}
fn main() {
    let stats = median_mode(&vec![1,2,3,4,5,6,7,8,9,10]);
    let stats2 = median_mode(&vec![1,2,3,4,6,7,8,9,10]);
    let stats3 = median_mode(&vec![1,2,3,4,6,7,3,9,10]);
    println!("{:#?}", stats);
    println!("{:#?}", stats2);
    println!("{:#?}", stats3);
}
