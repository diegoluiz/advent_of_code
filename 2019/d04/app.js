const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const input = fs.readFileSync(`${__dirname}/input.txt`, 'utf8').split('\n');

function part1() {
    return 0;
}

function part2() {
    return 0;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
