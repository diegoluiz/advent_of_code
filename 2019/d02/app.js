const { performance } = require('perf_hooks');

const fs = require('fs');
const { clone } = require('../helper');
const input = fs.readFileSync(`${__dirname}/input.txt`, 'utf8').split(',');

const compute = (array, limit = Number.MAX_SAFE_INTEGER) => {
    for (let i = 0; i < array.length; i += 4) {
        if (array[0] > limit) { return null; }

        const op = array[i];
        if (op === 1)
            array[array[i + 3]] = array[array[i + 1]] + array[array[i + 2]];
        else if (op === 2)
            array[array[i + 3]] = array[array[i + 1]] * array[array[i + 2]];
        else if (op === 99)
            return array[0];
    }
}

function part1() {
    const array = input.map(x => parseInt(x));
    array[1] = 12;
    array[2] = 2;

    return compute(array);
}

function part2() {
    const inputArr = input.map(x => parseInt(x));
    const target = 19690720;
    for (let i = 0; i < 100; i++) {
        for (let j = 0; j < 100; j++) {
            const array = clone(inputArr);
            array[1] = i;
            array[2] = j;

            const res = compute(array, target);
            if (res === target)
                return 100 * i + j
        }

    }
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
