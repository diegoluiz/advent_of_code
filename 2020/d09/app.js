const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d09/input.txt', 'utf8').split('\n');

const preamble = 25;

function part1() {
    const numbers = input.map(x => parseInt(x));

    for (let idx = preamble; idx < numbers.length; idx++) {
        const number = numbers[idx];

        let good = false;
        const s = new Set();
        for (let j = idx - preamble; j < idx; j++) {
            const tnumber = numbers[j];
            if (tnumber * 2 === number) { continue; }

            const enumber = number - tnumber;
            if (s.has(enumber)) {
                good = true;
                break;
            }

            s.add(tnumber);
        }

        if (!good) {
            return number;
        }
    }
}

function part2(invalidNumber) {
    const numbers = input.map(x => parseInt(x));

    let sum = 0;

    let i = 0;
    for (let f = 0; f < numbers.length; f++) {
        while (sum > invalidNumber && i < f - 1) {
            sum = sum - numbers[i];
            i++;
        }

        if (sum === invalidNumber) {
            const temp = numbers.slice(i, f);
            const min = Math.min(...temp);
            const max = Math.max(...temp);
            return min + max;
        }

        sum = sum + numbers[f];
    }

    return 0;
}

let start = performance.now();
const p1 = part1();
console.log(`Part 1: [${p1}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2(p1)}]. Time: ${performance.now() - start}`);
