/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findKthLargest = function (nums, k) {
    const pivotIndex = Math.floor(nums.length / 2);
    const pivot = nums.splice(pivotIndex, 1)[0];

    const left = [];
    const right = [];

    for (let i = 0; i < nums.length; i++) {
        if (pivot > nums[i]) {
            left.push(nums[i]);
        } else {
            right.push(nums[i]);
        }
    }

    // 如果右边元素的长度等于 k - 1，说明该值是第k大的元素
    if (right.length === k - 1) {
        return pivot;
    }

    // 长度大于该 k-1 时，说明在右边区域
    if (right.length > k -1 ) {
        return findKthLargest(right, k);
    }

    // 否则，在左边区域，此时找第 k 大，变为 k-right.length - 1 大
    return findKthLargest(left, k - 1 - right.length);
};


console.log(findKthLargest([-1,2,0], 2));