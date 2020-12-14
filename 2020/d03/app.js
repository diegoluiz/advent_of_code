const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d03/input.txt', 'utf8').split('\n');

function part1() {
    const grid = input
        .map(row => row.split(''));
    const gridHorLength = grid[0].length;

    let rowIndex = 0;
    let colIndex = 0;
    let treesNumber = 0;
    for (let i = 0; i < grid.length; i++) {
        if (grid[rowIndex][colIndex] === '#')
            treesNumber++;

        rowIndex += 1;
        colIndex = (colIndex + 3) % gridHorLength;

    }

    return treesNumber;
}

function part2() {
    const grid = input
        .map(row => row.split(''));
    const gridHorLength = grid[0].length;

    const patterns = [
        { x: 0, y: 0, stepX: 1, stepY: 1, treeNumber: 0 },
        { x: 0, y: 0, stepX: 3, stepY: 1, treeNumber: 0 },
        { x: 0, y: 0, stepX: 5, stepY: 1, treeNumber: 0 },
        { x: 0, y: 0, stepX: 7, stepY: 1, treeNumber: 0 },
        { x: 0, y: 0, stepX: 1, stepY: 2, treeNumber: 0 },
    ];

    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < patterns.length; j++) {
            const pattern = patterns[j];

            if (grid[pattern.y] && grid[pattern.y][pattern.x] === '#')
                pattern.treeNumber++;

            pattern.y += pattern.stepY;
            pattern.x = (pattern.x + pattern.stepX) % gridHorLength;
        }
    }

    return patterns.map(x => x.treeNumber).reduce((prev, curr) => prev * curr, 1);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
