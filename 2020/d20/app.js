const { performance } = require('perf_hooks');

const _ = require('lodash');
const fs = require('fs');

const input = fs.readFileSync('d20/input.txt', 'utf8');

function rev(str) {
    return str.split('').reverse().join('');
}

function part1() {
    const tiles = input
        .split('\n\n')
        .filter(x => x)
        .map(tile => {
            const [id, content] = tile.split(':');
            const grid = content.split('\n').filter(x => x).map(x => x.split(''));

            const t = grid[0].join('');
            const b = grid[grid.length - 1].join('');
            const r = grid.map(x => x[0]).join('');
            const l = grid.map(x => x[x.length - 1]).join('');

            const lines = { t, b, l, r };
            const revLines = {
                revT: rev(t),
                revB: rev(b),
                revL: rev(l),
                revR: rev(r)
            };

            return {
                id: BigInt(parseInt(id.substring(5))),
                grid,
                lines,
                revLines,
                allEdges: Object.assign(lines, revLines)
            };
        });

    const edgesCache = {};

    tiles.forEach(tile => {
        _.forEach(tile.allEdges, (line) => {
            if (!edgesCache[[line]])
                edgesCache[[line]] = 0;

            edgesCache[line]++;
        });
    });

    const corners = tiles.filter(tile => {
        let singleEdge = 0;
        _.forEach(tile.allEdges, (line) => {
            if (edgesCache[[line]] == 1) {
                singleEdge++;
            }
        });

        return singleEdge == 4;
    });

    return corners.reduce((p, c) => p * c.id, 1n);
}

function part2() {
    const transpose = (input) => {
        const m = JSON.parse(JSON.stringify(input));
        return m[0].map((x, i) => m.map(x => x[i]));
    };

    const vReverse = (m) => {
        return JSON.parse(JSON.stringify(m)).reverse();
    };

    const hReverse = (m) => {
        return JSON.parse(JSON.stringify(m)).map(x => x.reverse());
    };

    const rotate = (m) => {
        return vReverse(transpose(m));
    };

    const buildEdges = (grid) => {
        const t = grid[0].join('');
        const b = grid[grid.length - 1].join('');
        const l = grid.map(x => x[0]).join('');
        const r = grid.map(x => x[x.length - 1]).join('');

        return {
            t, b, l, r,
            revT: rev(t), revB: rev(b), revL: rev(l), revR: rev(r)
        };
    };

    const inputTiles = input
        .split('\n\n')
        .filter(x => x)
        .map(tile => {
            const [id, content] = tile.split(':');
            const grid = content.split('\n').filter(x => x).map(x => x.split(''));

            return {
                id: BigInt(parseInt(id.substring(5))),
                grid,
                lines: buildEdges(grid),
            };
        });

    const tiles = inputTiles.map(tile => {
        const neighbours = inputTiles
            .filter(x => _.some(x.lines, te => _.some(tile.lines, se => se === te)))
            .filter(x => x.id !== tile.id);

        return Object.assign(tile, { neighbours });
    });

    const edgesCache = {};

    tiles.forEach(tile => {
        _.forEach(tile.lines, (line) => {
            if (!edgesCache[[line]])
                edgesCache[[line]] = 0;

            edgesCache[line]++;
        });
    });

    const sideSize = Math.sqrt(tiles.length);
    const grid = [];

    let firstTile = tiles.find(x => x.neighbours.length === 2);

    while (edgesCache[[firstTile.lines.b]] !== 2 || edgesCache[[firstTile.lines.r]] !== 2) {
        firstTile.grid = rotate(firstTile.grid);
        firstTile.lines = buildEdges(firstTile.grid);
    }
    grid.push([firstTile.grid]);

    for (let i = 0; i < sideSize; i++) {
        let tile = firstTile;
        for (let j = 1; j < sideSize; j++) {
            const rightEdge = tile.lines.r;
            let rightNeighbour = tile.neighbours.find(x => _.some(x.lines, y => y === rightEdge));

            while (rightNeighbour.lines.l != rightEdge && rightNeighbour.lines.revL != rightEdge) {
                rightNeighbour.grid = rotate(rightNeighbour.grid);
                rightNeighbour.lines = buildEdges(rightNeighbour.grid);
            }

            if (rightNeighbour.lines.revL == rightEdge) {
                rightNeighbour.grid = vReverse(rightNeighbour.grid);
                rightNeighbour.lines = buildEdges(rightNeighbour.grid);
            }

            tile = rightNeighbour;
            grid[i].push(tile.grid);
        }

        const bottomEdge = firstTile.lines.b;
        let bottomNeighbour = firstTile.neighbours.find(x => _.some(x.lines, y => y === bottomEdge));
        if (!bottomNeighbour) {
            break;
        }

        while (bottomNeighbour.lines.t != bottomEdge && bottomNeighbour.lines.revT != bottomEdge) {
            bottomNeighbour.grid = rotate(bottomNeighbour.grid);
            bottomNeighbour.lines = buildEdges(bottomNeighbour.grid);
        }

        if (bottomNeighbour.lines.revT == bottomEdge) {
            bottomNeighbour.grid = hReverse(bottomNeighbour.grid);
            bottomNeighbour.lines = buildEdges(bottomNeighbour.grid);
        }

        firstTile = bottomNeighbour;
        grid.push([firstTile.grid]);
    }

    let alllines = [];
    grid.forEach(gy => {
        const lines = [...Array(8)].map(() => []);
        gy.forEach((gx) => {
            gx.slice(1, -1).forEach((y, i) => {
                lines[i] = lines[i].concat(y.slice(1, -1));
            });
        });
        alllines = alllines.concat(lines);
    });
    const hashesNumber = alllines.reduce((p, c) => p + c.filter(x => x === '#').length, 0);

    const fullMap1 = alllines;
    const fullMap2 = rotate(fullMap1);
    const fullMap3 = rotate(fullMap2);
    const fullMap4 = rotate(fullMap3);
    const fullMap5 = vReverse(fullMap1);
    const fullMap6 = vReverse(fullMap2);
    const fullMap7 = vReverse(fullMap3);
    const fullMap8 = vReverse(fullMap4);

    const fullMaps = [
        fullMap1.map(x => x.join('')),
        fullMap2.map(x => x.join('')),
        fullMap3.map(x => x.join('')),
        fullMap4.map(x => x.join('')),
        fullMap5.map(x => x.join('')),
        fullMap6.map(x => x.join('')),
        fullMap7.map(x => x.join('')),
        fullMap8.map(x => x.join(''))
    ];

    // const fs = require('fs');
    // fs.writeFileSync('./map.txt', fullMaps[0].join('\n'));

    const baseMonster = [
        '                  # ',
        '#    ##    ##    ###',
        ' #  #  #  #  #  #   '];

    const monsterLength = baseMonster[0].length;

    const monsterMap = _.flatMap(baseMonster, (l, r) => {
        return l.split('')
            .map((v, i) => ({ v, i }))
            .filter(x => x.v === '#')
            .map(x => ({ r, c: x.i }));
    });

    let monsterCounts = [];
    fullMaps.forEach(fullMap => {
        let monsterCount = 0;
        for (let row = 0; row < fullMap.length - 2; row++) {
            const line = fullMap[row];
            for (let col = 0; col < line.length - monsterLength; col++) {
                const a = monsterMap.every((v) => {
                    return fullMap[row + v.r][col + v.c] === '#';
                });

                if (a){
                    monsterCount++;
                }
            }
        }
        
        monsterCounts.push(monsterCount);
    });

    const monsterCount = Math.max(...monsterCounts);
    return hashesNumber - (monsterCount * monsterMap.length);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
