const { performance } = require('perf_hooks');

const fs = require('fs');
const input = fs.readFileSync('d12/input.txt', 'utf8').split('\n');


function part1() {
    const actions = input.map(x => ({ a: x[0], l: parseInt(x.substring(1)) }));

    const dirs = { N: 0, E: 1, S: 2, W: 3 };
    const dirsMap = ['N', 'E', 'S', 'W'];

    let d = 'E';
    let h = 0;
    let v = 0;

    const history = [];
    actions.forEach(action => {
        let a = action.a;

        if (a === 'L' || a === 'R') {
            const steps = (action.l / 90) * (a === 'R' ? 1 : -1);
            const newDir = ((dirs[d] + steps) + 4) % 4;
            d = dirsMap[newDir];
        }

        if (a === 'F') {
            a = d;
        }

        switch (a) {
            case 'N':
                v += action.l;
                break;
            case 'S':
                v -= action.l;
                break;
            case 'E':
                h += action.l;
                break;
            case 'W':
                h -= action.l;
                break;
        }

        history.push({ v, h, d });
    });

    return Math.abs(v) + Math.abs(h);
}

function part2() {
    const actions = input.map(x => ({ a: x[0], l: parseInt(x.substring(1)) }));

    const ship = { h: 0, v: 0 };
    const waypoint = { h: 10, v: 1 };

    const history = [];
    actions.forEach(action => {
        switch (action.a) {
            case 'R': {
                const steps = ((action.l / 90) + 4) % 4;

                for (let i = 0; i < steps; i++) {
                    const v = waypoint.v;
                    const h = waypoint.h;

                    waypoint.h = v;
                    waypoint.v = -h;
                }

                break;
            }
            case 'L': {
                const steps = ((action.l / 90) + 4) % 4;

                for (let i = 0; i < steps; i++) {
                    const v = waypoint.v;
                    const h = waypoint.h;

                    waypoint.h = -v;
                    waypoint.v = h;
                }

                break;
            }
            case 'F': {
                const v = waypoint.v * action.l;
                const h = waypoint.h * action.l;
                ship.v += v;
                ship.h += h;
                break;
            }
            case 'N': {
                waypoint.v += action.l;
                break;
            }
            case 'S': {
                waypoint.v -= action.l;
                break;
            }
            case 'E': {
                waypoint.h += action.l;
                break;
            }
            case 'W': {
                waypoint.h -= action.l;
                break;
            }
        }

        history.push({
            ship: { v: ship.v, h: ship.h },
            waypoint: { v: waypoint.v, h: waypoint.h }
        });
    });

    return Math.abs(ship.v) + Math.abs(ship.h);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
