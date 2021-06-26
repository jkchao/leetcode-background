#[allow(dead_code)]
pub fn merge_sorted_arrays(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut t = m + n - 1;
    while t >= 0 {
        if i < 0 {
            nums1[t as usize] = nums2[j as usize];
            j -= 1;
        } else if j < 0 {
            nums1[t as usize] = nums1[i as usize];
            i -= 1;
        } else if nums1[i as usize] > nums2[j as usize] {
            nums1[t as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[t as usize] = nums2[j as usize];
            j -= 1;
        }

        t -= 1;
    }
}

#[test]
fn internal() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    merge_sorted_arrays(&mut nums1, 3, vec![2, 5, 6], 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
