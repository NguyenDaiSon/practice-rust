impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left_pos, mut right_pos) = (0, nums.len() as i32 - 1);
        let mut result_pos = 0;

        while left_pos <= right_pos {
            let middle_pos = left_pos + (right_pos - left_pos)/2;
            if Solution::move_left(&nums, middle_pos) {
                result_pos = middle_pos;
                right_pos = middle_pos - 1;
            } else {
                left_pos = middle_pos + 1;
            }
        }
        
        nums[result_pos as usize]
    }

    pub fn move_left(nums: &Vec<i32>, index: i32) -> bool {
        if index == nums.len() as i32 - 1 {
            return true;
        }

        if index % 2 == 1 {
            return nums[index as usize] != nums[index as usize - 1];
        }

        return nums[index as usize] != nums[index as usize + 1];
    }
}
