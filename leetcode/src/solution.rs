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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
}
