/**
 * @param {number} target
 * @param {number[]} nums
 * @return {number}
 */
 var minSubArrayLen = function(target, nums) {

    let res = Number.MAX_SAFE_INTEGER;

    let i = 0;
    let j = 0;
    let sum = 0;

    while(j < nums.length) {
        sum += nums[j];

        while(sum >= target) {
            res = Math.min(res, j - i + 1);
            sum -= nums[i];
            i++;
        }

        j++;
    }
    return res === Number.MAX_SAFE_INTEGER ? 0 : res;
};


console.log(minSubArrayLen(7, [2,3,1,2,4,3]));
console.log(minSubArrayLen(4, [1,4,4]));
console.log(minSubArrayLen(11, [1,1,1,1,1,1,1,1]));
console.log(minSubArrayLen(11, [1,2,3,4,5]));