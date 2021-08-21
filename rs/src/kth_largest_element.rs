

#[allow(dead_code)]
pub fn kth_largest_element(nums: Vec<i32>, target: i32) -> i32 {

    let middle_index = nums.len() / 2;
    let middle = nums[middle_index];

    let _transform_nums = [&nums[..middle_index],&nums[middle_index + 1..]].concat();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for i in _transform_nums {
        if middle >= i {
            left.push(i);
        }

        if middle < i {
            right.push(i);
        }
    }

    if right.len() == (target - 1) as usize {
        return middle;
    }

    if right.len() > (target - 1) as usize {
        return kth_largest_element(right, target);
    }

    return kth_largest_element(left, target - 1 - right.len() as i32);

}

#[test]
fn test() {
    let res = kth_largest_element(vec![3,2,1,5,6,4], 2);
    assert_eq!(res, 5);

    let res = kth_largest_element(vec![3,2,3,1,2,4,5,5,6], 4);
    assert_eq!(res, 4);
}