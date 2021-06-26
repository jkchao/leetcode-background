use std::collections::HashMap;

#[allow(dead_code)]
pub fn sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    map.insert(nums[0], 0);

    for (i, item) in nums.iter().enumerate() {
        let rest = target - item;
        if let Some(&j) = map.get(&rest) {
            if j != i {
                return vec![j as i32, i as i32];
            }
        };

        map.insert(*item, i);
    }

    panic!()
}

#[test]
fn internal() {
    let sum1 = sum(vec![1, 2, 3], 3);
    assert_eq!(sum1, vec![0, 1]);

    let sum2 = sum(vec![3, 2, 4], 6);
    assert_eq!(sum2, vec![1, 2]);

    let sum3 = sum(vec![3, 3], 6);
    assert_eq!(sum3, vec![0, 1]);
}
