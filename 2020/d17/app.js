const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d17/input.txt', 'utf8').split('\n');

const part1GetAdjacentCords = (grid, sx, sy, sz) => {

    let cords = [];
    for (let z = sz - 1; z <= sz + 1; z++) {
        if (z < 0 || z >= grid.length) continue;
        for (let y = sy - 1; y <= sy + 1; y++) {
            if (y < 0 || y >= grid[0].length) continue;
            for (let x = sx - 1; x <= sx + 1; x++) {
                if (x < 0 || x >= grid[0][0].length) continue;
                if (x != sx || y != sy || z != sz)
                    cords.push({ z, y, x });
            }
        }
    }

    return cords;
};

function part1() {
    let inputLayer = input.map(x => x.split(''));
    const sizeX = input[0].length + 15;
    const sizeY = input.length + 15;
    const sizeZ = 1 + 15;

    let grid = [];

    for (let z = 0; z < sizeZ; z++) {
        grid[z] = [];
        for (let y = 0; y < sizeY; y++) {
            grid[z][y] = [];
            for (let x = 0; x < sizeX; x++) {
                grid[z][y][x] = '.';
            }
        }
    }

    let startZ = Math.floor(sizeZ / 2);
    let startX = Math.floor((sizeX / 2) - (inputLayer[0].length / 2));
    let startY = Math.floor((sizeY / 2) - (inputLayer[0].length / 2));

    inputLayer.forEach((y, idxY) => {
        y.forEach((x, idxX) => {
            grid[startZ][startY + idxY][startX + idxX] = x;
        });
    });

    const rule = (grid, cords) => {
        let sum = 0;
        cords.forEach(cord => {
            if (grid[cord.z][cord.y][cord.x] === '#') {
                sum++;
            }
        });
        return sum;
    };

    for (let cycle = 0; cycle < 6; cycle++) {
        const gridClone = JSON.parse(JSON.stringify(grid));

        grid.forEach((layer, z) => {
            layer.forEach((line, y) => {
                line.forEach((item, x) => {
                    const adjacentCords = part1GetAdjacentCords(grid, x, y, z);
                    const sum = rule(grid, adjacentCords);
                    if (sum < 2 || sum > 3) {
                        gridClone[z][y][x] = '.';
                    }
                    if (item === '.' && sum === 3) {
                        gridClone[z][y][x] = '#';
                    }
                });
            });
        });

        grid = gridClone;
    }

    let s = 0;
    grid.forEach((layer) => {
        layer.forEach((line) => {
            line.forEach((item) => s += item === '#' ? 1 : 0);
        });
    });

    return s;
}

const part2GetAdjacentCords = (grid, sx, sy, sz, sw) => {

    let cords = [];

    for (let w = sw - 1; w <= sw + 1; w++) {
        if (w < 0 || w >= grid.length) continue;
        for (let z = sz - 1; z <= sz + 1; z++) {
            if (z < 0 || z >= grid[0].length) continue;
            for (let y = sy - 1; y <= sy + 1; y++) {
                if (y < 0 || y >= grid[0][0].length) continue;
                for (let x = sx - 1; x <= sx + 1; x++) {
                    if (x < 0 || x >= grid[0][0][0].length) continue;
                    if (x != sx || y != sy || z != sz || w != sw)
                        cords.push({ w, z, y, x });
                }
            }
        }
    }
    return cords;
};


function part2() {
    let inputLayer = input.map(x => x.split(''));
    const sizeX = input[0].length + 15;
    const sizeY = input.length + 15;
    const sizeZ = 1 + 15;
    const sizeW = 1 + 15;

    let grid = [];

    for (let w = 0; w < sizeW; w++) {
        grid[w] = [];
        for (let z = 0; z < sizeZ; z++) {
            grid[w][z] = [];
            for (let y = 0; y < sizeY; y++) {
                grid[w][z][y] = [];
                for (let x = 0; x < sizeX; x++) {
                    grid[w][z][y][x] = '.';
                }
            }
        }
    }

    let startW = Math.floor(sizeZ / 2);
    let startZ = Math.floor(sizeZ / 2);
    let startX = Math.floor((sizeX / 2) - (inputLayer[0].length / 2));
    let startY = Math.floor((sizeY / 2) - (inputLayer[0].length / 2));

    inputLayer.forEach((y, idxY) => {
        y.forEach((x, idxX) => {
            grid[startW][startZ][startY + idxY][startX + idxX] = x;
        });
    });

    const rule = (grid, cords) => {
        let sum = 0;
        cords.forEach(cord => {
            if (grid[cord.w][cord.z][cord.y][cord.x] === '#') {
                sum++;
            }
        });
        return sum;
    };

    for (let cycle = 0; cycle < 6; cycle++) {
        const gridClone = JSON.parse(JSON.stringify(grid));

        grid.forEach((dim, w) => {
            dim.forEach((layer, z) => {
                layer.forEach((line, y) => {
                    line.forEach((item, x) => {
                        const adjacentCords = part2GetAdjacentCords(grid, x, y, z, w);
                        const sum = rule(grid, adjacentCords);
                        if (sum < 2 || sum > 3) {
                            gridClone[w][z][y][x] = '.';
                        }
                        if (item === '.' && sum === 3) {
                            gridClone[w][z][y][x] = '#';
                        }
                    });
                });
            });
        });

        grid = gridClone;
    }

    let s = 0;
    grid.forEach((dim) => {
        dim.forEach((layer) => {
            layer.forEach((line) => {
                line.forEach((item) => s += item === '#' ? 1 : 0);
            });
        });
    });

    return s;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
