use std::collections::HashMap;

fn main() {
    // Display the message "Hello, world!"
    let nums = vec![2, 7, 11, 15];
    let res = two_sum(nums, 18);
    println!("{:?}", res);
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::with_capacity(nums.len());

    for (i, vi) in nums.iter().enumerate() {
        match hash.get(&(target - vi)) {
            Some(j) => return vec![*j as i32, i as i32],
            None => {
                hash.insert(vi, i);
            }
        }
    }
    panic!("No answer");
}

fn _two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    for index in 0..nums.len() {
        let n = target - nums[index];
        if map.contains_key(&n) {
            return vec![*map.get(&n).unwrap(), index as i32];
        } else {
            map.insert(nums[index], index as i32);
        }
        println!("{index} -- {}", nums[index]);
    }
    unreachable!();
}
