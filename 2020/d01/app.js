const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d01/input.txt', 'utf8').split('\n');

const target = 2020;

function part1() {
    const map = {};
    for (let index = 0; index < input.length; index++) {
        const curr = parseInt(input[index]);
        const expected = target - curr;
        if (map[expected]) {
            return (curr * expected);
        }
        map[curr] = 1;
    }
}

function part2() {
    const map = {};
    const nums = [];

    for (let index = 0; index < input.length; index++) {
        const curr = parseInt(input[index]);
        nums.push(curr);
        map[curr] = 1;
    }

    nums.sort((a, b) => a - b);

    let ini = 0;
    let fin = nums.length - 1;

    for (ini = 0; ini < nums.length && ini <= fin; ini++) {
        while (nums[ini] + nums[fin] > target && ini <= fin) {
            fin--;
        }

        for (let fin2 = fin; fin2 > ini; fin2--) {
            const expected = target - nums[ini] - nums[fin2];
            if (map[expected]) {
                return (nums[ini] * nums[fin2] * expected);
            }
        }
    }
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
