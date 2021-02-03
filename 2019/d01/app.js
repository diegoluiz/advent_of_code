const { performance } = require('perf_hooks');

const fs = require('fs');
const { sum } = require('../helper');
const input = fs.readFileSync(`${__dirname}/input.txt`, 'utf8').split('\n');

function part1() {
    const modules = input.map(x => parseInt(x));
    const fuelReqs = modules.map(x => Math.floor(x / 3) - 2);
    return sum(fuelReqs);
}

function part2() {
    const calcRec = f => {
        if (f <= 0) {
            return 0;
        }

        const r = Math.floor(f / 3) - 2;
        return r + calcRec(r);
    };

    const modules = input.map(x => parseInt(x));
    const fuelReqs = modules.map(x => calcRec(x));

    return sum(fuelReqs);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
