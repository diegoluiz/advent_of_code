const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d10/input.txt', 'utf8').split('\n');

function part1() {
    const numbers = input.map(x => parseInt(x)).sort((a, b) => a - b);

    const sums = {
        0: 0,
        1: 0,
        2: 0,
        3: 0,
    };
    let prev = 0;
    for (let idx = 0; idx < numbers.length; idx++) {
        const number = numbers[idx];
        const diff = number - prev;
        sums[diff]++;
        prev = number;
    }

    sums[3]++;

    return sums[3] * sums[1];
}


function part2() {
    let numbers = input.map(x => parseInt(x));
    numbers.push(0);
    numbers = numbers.sort((a, b) => a - b);
    numbers.push(numbers[numbers.length - 1] + 3);
    numbers = numbers.reverse();

    const map = {
        [numbers[0]]: 1
    };

    for (let idx = 1; idx < numbers.length; idx++) {
        const number = numbers[idx];
        let sum = 0;
        for (let drift = 1; drift <= 3; drift++) {
            const t = number + drift;
            if (map[[t]]) {
                sum += map[[t]];
            }
        }
        if (sum > 0) {
            map[[number]] = sum;
        }
    }

    return map[numbers[numbers.length - 1]];
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`)

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`)
