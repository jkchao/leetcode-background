
#[allow(dead_code)]
pub fn minimum_size_sub_array_sum(target: i32, nums: Vec<i32>) -> i32 {
    let mut res = i32::MAX;

    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    while j < nums.len() {
        sum += nums[j];

        while sum >= target {
            res = res.min((j - i + 1) as i32);
            sum -= nums[i];
            i += 1;
        }

        j += 1;
    }

    if res == i32::MAX {
        0
    } else {
        res
    }
}

#[test]
fn test() {
    let sum = minimum_size_sub_array_sum(7,vec![2,3,1,2,4,3]);
    assert_eq!(sum, 2);

    let sum = minimum_size_sub_array_sum(4,vec![1,4,4]);
    assert_eq!(sum, 1);

    let sum = minimum_size_sub_array_sum(11,vec![1,1,1,1,1,1,1,1]);
    assert_eq!(sum, 0);
}
