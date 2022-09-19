use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn two_sum_test() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 8;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![2, 4]);
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    nums.iter().enumerate().for_each(|(index, value)| {
        let expect = target - value;
        if hash_map.contains_key(&expect) {
            let i = hash_map.get(&expect).unwrap();
            result.extend_from_slice(&[*i as i32, index as i32]);
        } else {
            hash_map.insert(*value, index);
        }
    });
    result
}
