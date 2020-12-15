const { performance } = require('perf_hooks');

const fs = require('fs');
const input = fs.readFileSync('d15/input.txt', 'utf8').split('\n');

function part1() {
    const numbers = input[0]
        .split(',')
        .map(x => parseInt(x));

    const map = {};

    numbers.forEach((num, idx) => { map[num] = idx + 1; });

    let lastNumber = numbers[numbers.length - 1];
    map[lastNumber] = undefined;

    for (let turn = numbers.length + 1; turn <= 2020; turn++) {
        if (map[lastNumber] !== undefined) {
            const temp = map[lastNumber];
            map[lastNumber] = turn - 1;
            lastNumber = turn - 1 - temp;
        } else {
            map[lastNumber] = turn - 1;
            lastNumber = 0;
        }
    }

    return lastNumber;
}

function part2() {
    const numbers = input[0].split(',').map(x => parseInt(x));

    const map = new Map();

    numbers.slice(0, -1).forEach((num, idx) => map.set(num, idx + 1));

    let lastNumber = numbers[numbers.length - 1];

    for (let turn = numbers.length + 1; turn <= 30000000; turn++) {
        if (map.has(lastNumber)) {
            const temp = map.get(lastNumber);
            map.set(lastNumber, turn - 1);
            lastNumber = turn - 1 - temp;
        } else {
            map.set(lastNumber, turn - 1);
            lastNumber = 0;
        }
    }

    return lastNumber;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
