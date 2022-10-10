use std::collections::hash_map::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (curr_idx, num) in nums.into_iter().enumerate() {
        let curr_idx = curr_idx as i32;
        if let Some(&prev_idx) = map.get(&num) {
            return vec![prev_idx, curr_idx];
        }
        map.insert(target - num, curr_idx);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
}
