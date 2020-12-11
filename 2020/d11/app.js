const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d11/input.txt', 'utf8').split('\n');

const doNothing = 0;
const doEmpty = -1;
const doBusy = 1;

const getAdjacentCords = (grid, x, y) => {
    const maxY = grid.length - 1;
    const maxX = grid[0].length - 1;

    const cords = [];
    cords.push(x > 0 && y > 0 ? { x: x - 1, y: y - 1, xdrift: -1, ydrift: -1 } : null);
    cords.push(x > 0 ? { x: x - 1, y: y, xdrift: -1, ydrift: 0 } : null);
    cords.push(x > 0 && y < maxY ? { x: x - 1, y: y + 1, xdrift: -1, ydrift: 1 } : null);
    cords.push(x < maxX && y > 0 ? { x: x + 1, y: y - 1, xdrift: 1, ydrift: -1 } : null);
    cords.push(x < maxX ? { x: x + 1, y: y, xdrift: 1, ydrift: 0 } : null);
    cords.push(x < maxX && y < maxY ? { x: x + 1, y: y + 1, xdrift: 1, ydrift: 1 } : null);
    cords.push(y > 0 ? { x: x, y: y - 1, xdrift: 0, ydrift: -1 } : null);
    cords.push(y < maxY ? { x: x, y: y + 1, xdrift: 0, ydrift: 1 } : null);

    return cords.filter(x => x);
}

function part1() {
    let grid = input.map(x => x.split(''));
    const visited = new Set();
    visited.add(JSON.stringify(grid));

    const rule = (grid, cords) => {
        let sum = 0;
        cords.forEach(cord => {
            if (grid[cord.y][cord.x] === '#') {
                sum++;
            }
        });
        if (sum === 0) {
            return doBusy;
        }
        if (sum < 4) {
            return doNothing;
        }

        return doEmpty;
    }

    while (true) {
        let gridClone = JSON.parse(JSON.stringify(grid));
        for (let y = 0; y < grid.length; y++) {
            for (let x = 0; x < grid[y].length; x++) {
                const adjacentCords = getAdjacentCords(grid, x, y);

                if (grid[y][x] === '.') {
                    continue;
                }
                const r = rule(grid, adjacentCords);
                if (r === doBusy) {
                    gridClone[y][x] = '#';
                } else if (r === doEmpty) {
                    gridClone[y][x] = 'L';
                }
            }
        }
        grid = gridClone;
        const t = JSON.stringify(grid);
        if (visited.has(t)) {
            let sum = 0;
            grid.forEach(i => {
                i.forEach(j => {
                    if (j === '#') sum++;
                });
            });
            return sum;
        }

        visited.add(t);
    }
}

function part2() {
    let grid = input.map(x => x.split(''));
    const visited = new Set();
    visited.add(JSON.stringify(grid));

    const rule = (grid, cords) => {
        let sum = 0;

        cords.forEach(cord => {
            let found = false;
            while (!found) {
                if (!grid[cord.y] || !grid[cord.y][cord.x]) {
                    found = true;
                    break;
                }
                if (grid[cord.y][cord.x] === '#') {
                    sum++;
                }
                if (grid[cord.y][cord.x] !== '.') {
                    found = true;
                }
                cord.x += cord.xdrift;
                cord.y += cord.ydrift;
            }
        });

        if (sum === 0) {
            return doBusy;
        }
        if (sum < 5) {
            return doNothing;
        }

        return doEmpty;
    }

    while (true) {
        let gridClone = JSON.parse(JSON.stringify(grid));
        for (let y = 0; y < grid.length; y++) {
            for (let x = 0; x < grid[y].length; x++) {
                const adjacentCords = getAdjacentCords(grid, x, y);

                if (grid[y][x] === '.') {
                    continue;
                }
                const r = rule(grid, adjacentCords);
                if (r === doBusy) {
                    gridClone[y][x] = '#';
                } else if (r === doEmpty) {
                    gridClone[y][x] = 'L';
                }
            }
        }
        grid = gridClone;
        const t = JSON.stringify(grid);
        if (visited.has(t)) {
            let sum = 0;
            grid.forEach(i => {
                i.forEach(j => {
                    if (j === '#') sum++;
                });
            });
            return sum;
        }

        visited.add(t);
    }
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`)

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`)
