impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let first_index = Solution::find_index(&nums, target, true);
        let last_index = Solution::find_index(&nums, target, false);
        vec![first_index, last_index]
    }

    pub fn find_index(nums: &Vec<i32>, target: i32, first_index: bool) -> i32 {
        let (mut expected_pos, mut first_pos, mut last_pos) = (-1, 0, nums.len() as i32 -1);
        while first_pos <= last_pos {
            let middle_pos = first_pos + (last_pos - first_pos)/2;
            if target == nums[middle_pos as usize] {
                expected_pos = middle_pos;
                if first_index {
                    last_pos = middle_pos - 1;
                } else {
                    first_pos = middle_pos + 1;
                }
            } else if target < nums[middle_pos as usize] {
                last_pos = middle_pos - 1;
            } else {
                first_pos = middle_pos + 1;
            }
        }
        expected_pos
    }
}
