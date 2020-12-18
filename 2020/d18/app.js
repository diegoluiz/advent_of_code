const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d18/input.txt', 'utf8').split('\n');

function part1() {
    const calculate = (s) => {
        let res = null;
        let op = null;
        s.replace('(', '').replace(')', '').split(' ').forEach(c => {
            if (c === '') {
                return;
            }
            if (res === null) {
                res = parseInt(c);

            } else if (c === '+' || c === '*') {
                op = c;
            } else {
                res = op === '+' ? res + parseInt(c) : res * parseInt(c);
            }
        });

        return res;
    };

    let sum = 0;
    input.forEach(exp => {
        let t = exp;
        let e = t.indexOf(')');
        let s = t.lastIndexOf('(', e);

        while (e >= 0) {
            const m = t.substring(s, e + 1);

            const r = calculate(m);

            t = `${t.substring(0, s)} ${r}${t.substring(e + 1)}`;
            e = t.indexOf(')');
            s = t.lastIndexOf('(', e);
        }

        const r = calculate(t);
        sum += r;
    });

    return sum;
}

function part2() {

    const calculate = (s) => {
        s = s.replace('(', '').replace(')', '').split(' ').join('');
        let ops = /(\d+)\+(\d+)/.exec(s);

        while (ops) {
            let t = parseInt(ops[1]) + parseInt(ops[2]);
            s = s.replace(ops[0], t + '');
            ops = /(\d+)\+(\d+)/.exec(s);
        }

        ops = /(\d+)\*(\d+)/.exec(s);
        while (ops) {
            let t = parseInt(ops[1]) * parseInt(ops[2]);
            s = s.replace(ops[0], t + '');
            ops = /(\d+)\*(\d+)/.exec(s);
        }

        const res = parseInt(s);
        return res;
    };

    let sum = 0;
    input.forEach(exp => {
        let t = exp;
        let e = t.indexOf(')');
        let s = t.lastIndexOf('(', e);

        while (e >= 0) {
            const m = t.substring(s, e + 1);

            const r = calculate(m);

            t = `${t.substring(0, s)} ${r}${t.substring(e + 1)}`;
            e = t.indexOf(')');
            s = t.lastIndexOf('(', e);
        }

        const r = calculate(t);
        sum += r;
    });

    return sum;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
