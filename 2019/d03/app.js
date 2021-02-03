const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const input = fs.readFileSync(`${__dirname}/input.txt`, 'utf8').split('\n');

let start = performance.now();

function part1(cross) {
    const crossDist = cross.map(n => Math.abs(n.x) + Math.abs(n.y)).sort((a, b) => a - b);
    return crossDist[1];
}

function part2() {
    return 0;
}

const getPoints = arr => {
    let x = 0, y = 0, ly = 0, lx = 0, t = 0;
    const points = [];
    arr.forEach(w => {
        if (w.a === 'R') x += w.d;
        if (w.a === 'L') x -= w.d;
        if (w.a === 'U') y += w.d;
        if (w.a === 'D') y -= w.d;

        t += (Math.abs(ly - y) + Math.abs(lx - x));
        for (let i = Math.min(x, lx); i <= Math.max(x, lx); i++) {
            points.push({ x: i, y, t });
        }

        for (let i = Math.min(y, ly); i <= Math.max(y, ly); i++) {
            points.push({ x, y: i, t });
        }

        ly = y;
        lx = x;
    });

    return points;
}

const [wire1, wire2] = input.map(wire => (
    wire.split(',').map(x => ({ a: x[0], d: parseInt(x.slice(1)) }))
));

const wire1Points = getPoints(wire1);
const wire2Points = getPoints(wire2);

const cross = _.intersectionWith(wire1Points, wire2Points, (w1, w2) => w1.x === w2.x && w1.y === w2.y);


console.log(`Part 1: [${part1(cross)}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
