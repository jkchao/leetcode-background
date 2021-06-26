#[allow(dead_code)]
pub fn merge_sorted_arrays(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
    let (mut m, mut n) = (m as usize, n as usize);

    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}

#[test]
fn internal() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    merge_sorted_arrays(&mut nums1, 3, vec![2, 5, 6], 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
