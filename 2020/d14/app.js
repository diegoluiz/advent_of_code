const { performance } = require('perf_hooks');

const fs = require('fs');
const input = fs.readFileSync('d14/input.txt', 'utf8').split('\n');
const _ = require('lodash');

String.prototype.replaceAt = function (index, replacement) {
    return this.substr(0, index) + replacement + this.substr(index + replacement.length);
}

function part1() {
    let mask = '';
    let memory = {};
    input.forEach(line => {
        if (line.startsWith('mask')) {
            mask = line.split('=')[1].trim();
            return;
        }

        const data = /^mem\[(\d+)\] \= (\d+)/.exec(line);
        const memId = data[1];
        const value = parseInt(data[2]).toString(2).padStart(36, '0');

        let currValue = memory[memId] || '0'.repeat(36);

        for (let i = currValue.length - 1; i >= 0; i--) {
            const m = mask[i];
            if (m != 'X') {
                currValue = currValue.replaceAt(i, m);
                continue;
            }

            if (!!value[i]) {
                currValue = currValue.replaceAt(i, value[i]);
            }
        }

        memory[memId] = currValue;
    });

    return Object.keys(memory).map(key => parseInt(memory[key], 2)).reduce((p, c) => p + c, 0);
}

function part2() {
    const calculateMemIds = (mask, memId) => {
        const memIdBinary = parseInt(memId).toString(2).padStart(36, '0');
        let result = memIdBinary;
        for (let i = mask.length - 1; i >= 0; i--) {
            const m = mask[i];
            if (m === '1' || m === 'X') {
                result = result.replaceAt(i, m);
            }
        }

        const explodeResult = (l) => {
            const i = l.indexOf('X');
            if (i < 0) {
                return [l];
            }

            return [
                explodeResult(l.replaceAt(i, '1')),
                explodeResult(l.replaceAt(i, '0')),
            ]
        }

        return _.flattenDeep(explodeResult(result));
    };

    let mask = '';
    let memory = {};
    input.forEach(line => {
        if (line.startsWith('mask')) {
            mask = line.split('=')[1].trim();
            return;
        }

        const data = /^mem\[(\d+)\] \= (\d+)/.exec(line);
        const memId = data[1];
        const value = parseInt(data[2]);

        const memIds = calculateMemIds(mask, memId).map(x => parseInt(x, 2));
        memIds.forEach(id => { memory[id] = value });
    });

    return Object.keys(memory).map(key => memory[key]).reduce((p, c) => p + c, 0);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`)

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`)
