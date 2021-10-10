// @ts-check

/**
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */

var addStrings = function (num1, num2) {
    let add = 0;
    let i = num1.length - 1;
    let j = num2.length - 1;
    const result = [];


    while (i >= 0 || j >= 0 || add !== 0) {
        const x = i >= 0 ? Number(num1.charAt(i)) : 0;
        const y = j >= 0 ? Number(num2.charAt(j)) : 0;
        const res = x + y + add;
        result.push(res % 10);
        add = Math.floor(res / 10);

        i--;
        j--;
    }
    return result.reverse().join("");
};
