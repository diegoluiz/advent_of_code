const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const input = fs.readFileSync('d24/input.txt', 'utf8').split('\n');

const cordstemp = {
    e: 'e',
    se: 'se',
    sw: 'sw',
    w: 'w',
    nw: 'nw',
    ne: 'ne'
};

function buildTileId(x, y) {
    return `${x}|${y}`;
}

class Tile {
    /**
     * @param {Number} x 
     * @param {Number} y 
     */
    constructor(x, y) {
        this.colour = 'w';
        this.x = x;
        this.y = y;
    }

    getId() {
        return buildTileId(this.x, this.y);
    }

    swapColour() {
        this.colour = this.colour === 'w' ? 'b' : 'w';
    }

    getNeighboursIds() {
        return [
            [this.x + 1, this.y],
            [this.x - 1, this.y],
            [this.x, this.y + 1],
            [this.x, this.y - 1],
            [this.x + 1, this.y - 1],
            [this.x - 1, this.y + 1],
        ].map(n => ({ x: n[0], y: n[1] }));
    }
}

function getTiles() {
    const list = input.map(line => {
        const cords = [];
        while (line) {
            if (line[0] === 'n' || line[0] === 's') {
                cords.push(cordstemp[[line.substring(0, 2)]]);
                line = line.substring(2);
            } else {
                cords.push(cordstemp[[line[0]]]);
                line = line.substring(1);
            }
        }
        return { cords };
    });

    const tilesMap = new Map();

    list.forEach(item => {
        let x = 0;
        let y = 0;

        item.cords.forEach(direction => {
            switch (direction) {
                case cordstemp.e:
                    x++;
                    break;
                case cordstemp.w:
                    x--;
                    break;
                case cordstemp.se:
                    y++;
                    break;
                case cordstemp.sw:
                    y++;
                    x--;
                    break;
                case cordstemp.ne:
                    y--;
                    x++;
                    break;
                case cordstemp.nw:
                    y--;
                    break;
            }
        });

        /** @type {Tile} */
        let t = tilesMap.get(buildTileId(x, y));
        if (t) {
            t.swapColour();
        } else {
            let tile = new Tile(x, y);
            tile.swapColour();
            tilesMap.set(tile.getId(), tile);
        }
    });

    return tilesMap;
}

function part1() {
    let tilesMap = getTiles();
    const blackTiles = Array.from(tilesMap.entries()).map(x => x[1]).filter(x => x.colour === 'b');
    return blackTiles.length;
}

function part2() {
    let tilesMap = getTiles();

    const getTile = (x, y) => {
        /** @type {Tile} */
        let t = tilesMap.get(buildTileId(x, y));
        if (t) {
            return t;
        }
        let tile = new Tile(x, y);
        tilesMap.set(tile.getId(), tile);
        return tile;
    };

    for (let day = 0; day < 100; day++) {
        const newMap = new Map();
        _.chain(Array.from(tilesMap.entries()))
            .map(x => x[1])
            .filter(x => x.colour === 'b')
            .map(n => [n.getNeighboursIds(), { x: n.x, y: n.y }])
            .flattenDeep()
            .map(id => getTile(id.x, id.y))
            .map(tile => {
                const blackNeighbours = tile
                    .getNeighboursIds()
                    .map(neighbour => tilesMap.get(buildTileId(neighbour.x, neighbour.y)))
                    .filter(x => x && x.colour === 'b')
                    .length;

                const newTile = new Tile(tile.x, tile.y);
                newTile.colour = tile.colour;
                if (tile.colour === 'b' && (blackNeighbours === 0 || blackNeighbours > 2)) {
                    newTile.colour = 'w';
                }
                if (tile.colour === 'w' && blackNeighbours === 2) {
                    newTile.colour = 'b';
                }
                if (newTile.colour !== 'b') {
                    return null;
                }
                return newTile;
            })
            .filter()
            .forEach(tile => newMap.set(tile.getId(), tile))
            .value();

        tilesMap = newMap;
    }
    const res = Array.from(tilesMap.entries()).filter(x => x[1].colour === 'b');
    return res.length;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
