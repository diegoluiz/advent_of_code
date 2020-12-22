const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const input = fs.readFileSync('d22/input.txt', 'utf8');


function part1() {
    const [player1Raw, player2Raw] = input.split('\n\n');

    const player1 = player1Raw.split('\n').slice(1).map(x => parseInt(x));
    const player2 = player2Raw.split('\n').slice(1).map(x => parseInt(x));

    while (player1.length !== 0 && player2.length !== 0) {
        const p1 = player1.shift();
        const p2 = player2.shift();

        if (p1 > p2) {
            player1.push(p1);
            player1.push(p2);
        } else {
            player2.push(p2);
            player2.push(p1);
        }
    }

    return _.chain(player1.concat(player2)).reverse().reduce((p, c, i) => p + (c * (i + 1))).value();
}

/**
 * 
 * @param {Array<Number>} player1 
 * @param {Array<Number>} player2 
 */
function part2RecGame(player1, player2) {
    const player1History = new Set();
    const player2History = new Set();


    while (player1.length !== 0 && player2.length !== 0) {
        if (player1History.has(JSON.stringify(player1))
            || player2History.has(JSON.stringify(player2))) {
            return { winner: 'p1', player1, player2 };
        }

        player1History.add(JSON.stringify(player1));
        player2History.add(JSON.stringify(player2));

        const p1 = player1.shift();
        const p2 = player2.shift();

        const subGame = (player1.length >= p1 && player2.length >= p2);
        let res = null;
        if (subGame) {
            res = part2RecGame(player1.slice(0, p1), player2.slice(0, p2));
        }

        if (res && res.winner === 'p1') {
            player1.push(p1);
            player1.push(p2);
        } else if (res && res.winner === 'p2') {
            player2.push(p2);
            player2.push(p1);
        } else if (p1 > p2) {
            player1.push(p1);
            player1.push(p2);
        } else {
            player2.push(p2);
            player2.push(p1);
        }
    }

    const winner = player1.length > player2.length ? 'p1' : 'p2';
    return { winner, player1, player2 };
}

function part2() {
    const [player1Raw, player2Raw] = input.split('\n\n');

    const player1 = player1Raw.split('\n').slice(1).map(x => parseInt(x));
    const player2 = player2Raw.split('\n').slice(1).map(x => parseInt(x));

    const res = part2RecGame(player1, player2);
    const t = res.winner === 'p1' ? res.player1 : res.player2;
    return _.chain(t).reverse().reduce((p, c, i) => p + (c * (i + 1))).value();
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
