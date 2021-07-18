// @ts-check

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findKthLargest = function (nums, k) {
    const arr = nums.sort((a, b) => b - a);
    return arr[k - 1];
};
