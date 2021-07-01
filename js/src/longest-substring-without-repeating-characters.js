// @ts-check

/**
 * @param {string} s
 * @return {number}
 */
function lengthOfLongestSubstring(s) {
    if (s.length === 0) {
        return 0;
    };

    if (s.length === 1) {
        return 1;
    }

    let set = new Set();
    let j = 1;
    let result = 0;

    set.add(s.charAt(0));

    for (let i = 0; i < s.length - 1; i++) {
        if (i !== 0) {
            set.delete(s.charAt(i - 1));
        };

        while(j < s.length) {
            if (set.has(s.charAt(j))) {
                break;
            } else {
                set.add(s.charAt(j));
            }
            j++;
        }

        result = Math.max(result, set.size);
    }
    return result;
}

lengthOfLongestSubstring("pwwkew");