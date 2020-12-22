const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d08/input.txt', 'utf8').split('\n');

const nop = 'nop';
const acc = 'acc';
const jmp = 'jmp';

function part1() {
    const insts = input.map(x => {
        const v = x.split(' ').filter(y => y);
        const ins = v[0];
        const val = parseInt(v[1]);

        return { ins, val };
    });

    let curr = 0;
    let accu = 0;
    const visited = new Set();
    while (true) {
        if (visited.has(curr)) {
            return accu;
        }

        visited.add(curr);
        const item = insts[curr];
        if (item.ins === nop) {
            curr++;
            continue;
        }
        if (item.ins === acc) {
            accu += item.val;
            curr++;
            continue;
        }
        if (item.ins === jmp) {
            curr += item.val;
            continue;
        }

        console.error('Something went horrible wrong... :( ');
        return null;
    }
}

function part2BruteForce(insts) {
    let curr = 0;
    let accu = 0;
    const visited = new Set();
    while (true) {
        if (visited.has(curr)) {
            return null;
        }

        if (curr >= insts.length) {
            return accu;
        }

        if (curr < 0) { //play safe
            return null;
        }

        visited.add(curr);
        const item = insts[curr];
        if (item.ins === nop) {
            curr++;
            continue;
        }
        if (item.ins === acc) {
            accu += item.val;
            curr++;
            continue;
        }
        if (item.ins === jmp) {
            curr += item.val;
            continue;
        }

        console.error('Something went horrible wrong... :( ');
        return null;
    }
}

function part2() {
    const insts = input.map(x => {
        const v = x.split(' ').filter(y => y);
        const ins = v[0];
        const val = parseInt(v[1]);

        return { ins, val };
    });

    for (let idx = 0; idx < insts.length; idx++) {
        const element = insts[idx];
        if (element === acc) {
            continue;
        }
        const n = element.ins === nop ? jmp : nop;

        const instsClone = JSON.parse(JSON.stringify(insts));

        instsClone[idx].ins = n;

        const res = part2BruteForce(instsClone);
        if (res != null) {
            return res;
        }
    }
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
