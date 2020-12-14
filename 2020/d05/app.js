const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d05/input.txt', 'utf8').split('\n');

function part1() {
    const seatIds = input
        .map(x => {
            return {
                row: x.substring(0, 7),
                col: x.substring(7, 10),
            };
        })
        .map(x => {
            const converted = {
                row: parseInt(x.row.replace(/F/g, '0').replace(/B/g, '1'), 2),
                col: parseInt(x.col.replace(/L/g, '0').replace(/R/g, '1'), 2),
            };

            converted.seatId = (converted.row * 8) + converted.col;

            return converted.seatId;
        });

    return Math.max(...seatIds);
}

function part2() {
    const seatIds = input
        .map(x => {
            return {
                row: x.substring(0, 7),
                col: x.substring(7, 10),
            };
        })
        .map(x => {
            const converted = {
                row: parseInt(x.row.replace(/F/g, '0').replace(/B/g, '1'), 2),
                col: parseInt(x.col.replace(/L/g, '0').replace(/R/g, '1'), 2),
            };

            converted.seatId = (converted.row * 8) + converted.col;

            return converted.seatId;
        })
        .sort();

    for (let seatIdx = 1; seatIdx < seatIds.length; seatIdx++) {
        const seatId = seatIds[seatIdx];
        const prevSeatId = seatIds[seatIdx - 1];
        if (seatId - 1 != prevSeatId) {
            return seatId - 1;
        }
    }

    return 0;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
