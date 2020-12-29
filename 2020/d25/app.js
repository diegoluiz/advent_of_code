const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const input = fs.readFileSync('d25/input.txt', 'utf8').split('\n');

function part1() {

    const [pub1, pub2] = input.map(x => parseInt(x));

    let loop1 = 0;
    let loop2 = 0;

    const divisor = 20201227;
    const subjectNumber = 7;

    let n = 1;
    while (n != pub1) {
        n *= subjectNumber;
        n %= divisor;
        loop1++;
    }

    n = 1;
    while (n != pub2) {
        n *= subjectNumber;
        n %= divisor;
        loop2++;
    }

    let enc1 = 1;
    for (let i = 0; i < loop1; i++) {
        enc1 *= pub2;
        enc1 %= divisor;
    }

    let enc2 = 1;
    for (let i = 0; i < loop2; i++) {
        enc2 *= pub1;
        enc2 %= divisor;
    }

    return enc1 || enc2; // either are correct
}

function part2() {
    // nothing to do here :)
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
