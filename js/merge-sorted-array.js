
// @ts-check

/**
 * @param {number[]} nums1
 * @param {number} m
 * @param {number[]} nums2
 * @param {number} n
 * @return {void} Do not return anything, modify nums1 in-place instead.
 */
function merge(nums1, m, nums2, n) {
    let i = m - 1;
    let j = n - 1;
    let t = m + n - 1;

    while(t >= 0) {
        if (i < 0) {
            nums1[t] = nums2[j];
            j --;
        }
        else if (j < 0) {
            nums1[t] = nums1[i];
            i --;
        }
        else if (nums1[i] > nums2[j]) {
            nums1[t] = nums1[i];
            i--;
        } else {
            nums1[t] = nums2[j];
            j--;
        }

        t --;
    }


    console.log(nums1);
}


merge([1,2,3,0,0,0], 3, [2,5,6], 3);