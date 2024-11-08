impl Solution {
    pub fn can_finish_eating(piles: &Vec<i32>, hours: i32, bananas: i32) -> bool {
        let mut hours_used = 0;
        for pile in piles {
            hours_used += (pile + bananas - 1) / bananas;
            if hours_used > hours {
                return false;
            }
        }

        (hours_used <= hours)
    }

    pub fn min_eating_speed(piles: Vec<i32>, hours: i32) -> i32 {
        let mut min_banana = 1;
        let mut max_banana = *piles.iter().max().unwrap();
        let mut min_speed = -1;
        while min_banana <= max_banana {
            let bananas = min_banana + (max_banana - min_banana) / 2;
            if Solution::can_finish_eating(&piles, hours, bananas) {
                min_speed = bananas;
                max_banana = bananas - 1;
            } else {
                min_banana = bananas + 1;
            }
        }

        min_speed
    }
}
