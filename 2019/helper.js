module.exports = {
    /** @type {Array<Number>} */
    sum: ([...arr]) => {
        return arr.reduce((p, c) => p + c, 0);
    },

    /** @type {Array<Number>} */
    multiply: ([...arr]) => {
        return arr.reduce((p, c) => p * c, 1);
    },

    clone: (x) => JSON.parse(JSON.stringify(x))
}