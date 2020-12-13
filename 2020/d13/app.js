const { performance } = require('perf_hooks');

const fs = require('fs');
const input = fs.readFileSync('d13/input.txt', 'utf8').split('\n');


function part1() {
    const arrival = input[0];
    const busTimes = input[1].split(',').filter(x => x !== 'x').map(x => parseInt(x));

    const closestTimes = busTimes
        .map(x => ({ id: x, arrialDiff: x - (arrival % x) }))
        .sort((a, b) => a.arrialDiff - b.arrialDiff);

    return closestTimes[0].id * closestTimes[0].arrialDiff;
}

function part2() {
    // source: https://www.geeksforgeeks.org/chinese-remainder-theorem-set-2-implementation/
    //         https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
    // I really didn't know this :(

    const modInverse = (a, m) => {
        const mod = a % m;
        for (let x = 1n; x < m; x++) if ((mod * x) % m == 1) return x;
        return 1;
    }

    let busTimes = input[1].split(',')
        .map((x, idx) => ({ id: x, pos: idx }))
        .filter(x => x.id !== 'x')
        .map(x => ({
            id: BigInt(parseInt(x.id)),
            pos: BigInt(x.pos),
        }));

    const product = busTimes.reduce((p, c) => p * c.id, 1n);

    const result = busTimes
        .map((x) => ({
            id: x.id,
            mod: (x.id - (x.pos % x.id)),
            pp: product / x.id
        })).map(x => {
            const modInv = modInverse(x.pp, x.id);
            return x.mod * modInv * x.pp;
        }).reduce((p, c) => p + c, 0n);
    return result % product;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`)

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`)
