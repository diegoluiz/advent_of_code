const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d06/input.txt', 'utf8').split('\n');

function part1() {
    let group = {};
    const groups = [];
    for (let groupIdx = 0; groupIdx < input.length; groupIdx++) {
        const line = input[groupIdx];
        if (line.trim() === '') {
            groups.push(group);
            group = {};
        }

        const letters = line.split('');
        letters.forEach(l => group[[l]] = 1);
    }
    if (group !== '') {
        groups.push(group);
    }

    return groups
        .map(group => Object.keys(group).length)
        .reduce((prev, curr) => prev + curr, 0);
}

function part2() {
    let group = { count: 0 };
    const groups = [];
    for (let groupIdx = 0; groupIdx < input.length; groupIdx++) {
        const line = input[groupIdx];
        if (line.trim() === '') {
            groups.push(group);
            group = { count: 0 };
        }

        const letters = line.split('');
        if (letters.length > 0) {
            group.count++;
        }
        letters.forEach(l => group[[l]] ? group[[l]]++ : group[[l]] = 1);
    }
    if (group !== '') {
        groups.push(group);
    }

    return groups
        .map(group => {
            return Object.keys(group)
            .filter(key => {
                if (key === 'count') {
                    return false;
                }
                return group[key] === group.count;
            }).length;
        })
        .reduce((prev, curr) => prev + curr, 0);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
