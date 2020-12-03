const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d02/input.txt', 'utf8').split('\n');

const target = 2020;

function part1() {
    // 2-6 c: fcpwjqhcgtffzlbj
    const reg = /^(\d+)\-(\d+) (\w): (\S+)$/
    return input
        .map(x => {
            const [v, min, max, letter, pass] = reg.exec(x);
            return { min, max, letter, pass };
        })
        .map(item => {
            const occurs = [...item.pass.matchAll(new RegExp(item.letter, 'g'))].length;
            return item.min <= occurs && occurs <= item.max ? 1 : 0;
        })
        .reduce((prev, curr) => prev+curr, 0);
}

function part2() {
    const reg = /^(\d+)\-(\d+) (\w): (\S+)$/
    return input
        .map(x => {
            const [v, first, second, letter, pass] = reg.exec(x);
            return { first, second, letter, pass };
        })
        .map(item => {
            const a = (item.letter == item.pass[item.first - 1]) != (item.letter == item.pass[item.second - 1]);
            return a  ? 1 : 0;
        })
        .reduce((prev, curr) => prev+curr, 0);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`)

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`)
