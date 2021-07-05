#[allow(dead_code)]
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut pre = nums[0];

    for i in 1..nums.len() {
        pre = (pre + nums[i]).max(nums[i]);
        res = res.max(pre);
    }
    res
}

#[test]
fn internal() {
    let res = max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    assert_eq!(res, 6);

    let res = max_sub_array(vec![5, 4, -1, 7, 8]);
    assert_eq!(res, 23);
}
