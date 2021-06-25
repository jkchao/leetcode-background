const twoSum = function(nums, target) {
    const map = new Map();
    map.set(nums[0], 0);

    for (let i = 1; i < nums.length; i++) {
        const res = target - nums[i];

        if (map.get(res) !== undefined) {
            return [map.get(res), i];
        }
        map.set(nums[i], i);
    }
};