
// @ts-check

/**
 * @param {string} version1
 * @param {string} version2
 * @return {number}
 */
 var compareVersion = function(version1, version2) {
    let v1 = version1.split('.');
    let v2 = version2.split('.');

    const l1 = v1.length;
    const l2 = v2.length;

    let i = 0;
    while(i < l1 || i < l2) {
        const n1 = Number(v1[i] || 0);
        const n2 = Number(v2[i] || 0);

        if (v1[i] > v2[i]) {
            return 1;
        }

        if (n1 < n2) {
            return -1;
        }

        i++;

    }

    return 0;
};

console.log(compareVersion("1.0.1", "1"))
console.log(compareVersion("7.5.2.4", "7.5.3"))