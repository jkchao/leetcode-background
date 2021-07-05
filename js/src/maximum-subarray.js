// @ts-check

/**
 * @param {number[]} nums
 * @return {number}
 */
function maxSubArray(nums) {
    if (nums.length === 1) {
        return nums[0];
    }
    let sum = nums[0];
    let pre = nums[0];

    for (let i = 1; i < nums.length; i++) {
        pre = Math.max(pre + nums[i], nums[i]);
        sum = Math.max(sum, pre);
    }

    return sum;
}

const result = maxSubArray([5, 4, -1, 7, 8]);
console.log(result);
