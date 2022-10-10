use std::collections::hash_map::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());

    nums.into_iter()
        .enumerate()
        .find_map(|(i, n)| {
            let rest = target - n;

            // if there is a rest num in map,
            // return the index of rest num and current num
            if let Some(&j) = map.get(&rest) {
                Some(vec![j as i32, i as i32])
            } else {
                map.insert(n, i);
                None
            }
        })
        .unwrap_or_default()
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
