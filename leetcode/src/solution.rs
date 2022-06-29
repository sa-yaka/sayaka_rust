use std::{collections::HashMap, iter};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        let left = i;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] + nums[right] == target {
                return vec![left as i32, right as i32];
            }
            right -= 1;
        }
    }

    Vec::new()
}

pub fn _roman_to_int(s: String) -> i32 {
    let map = ['I', 'V', 'X', 'L', 'C', 'D', 'M']
        .iter()
        .zip([1, 5, 10, 50, 100, 500, 100])
        .collect::<HashMap<_, _>>();
    s.chars().rev().fold(0, |acc, c| {
        acc + if acc > 4 * map[&c] { -map[&c] } else { map[&c] }
    })
}

// 左边大于等与右边就加  右边小于左边就减
fn fold(acc: i32, c: i32) {
    if acc > 4 * map[&c] {
        acc - map[&c]
    } else {
        acc + map[&c]
    }
}
