const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d04/input.txt', 'utf8').split('\n');

function part1() {
    let passportLine = '';
    const passportLines = [];
    for (let lineIdx = 0; lineIdx < input.length; lineIdx++) {
        const line = input[lineIdx];
        if (line.trim() === '') {
            passportLines.push(passportLine);
            passportLine = '';
        }

        passportLine += ' ' + line;
    }
    if (passportLine !== '') {
        passportLines.push(passportLine);
    }

    const passports = passportLines
        .map(x => {
            const props = x
                .split(' ')
                .filter(y => !!y)
                .map(y => {
                    const prop = y.split(':');
                    return { key: [prop[0]], value: prop[1] };
                })
                .reduce((prev, curr) => Object.assign(prev, { [curr.key]: curr.value }), {});
            return props;
        });

    return passports
        .map(passport => {
            if (Object.keys(passport).length < 7) {
                return 0;
            }

            if (Object.keys(passport).length === 7 && passport.cid) {
                return 0;
            }

            return 1;
        })
        .reduce((prev, curr) => prev + curr, 0);
}

function part2() {
    let passportLine = '';
    const passportLines = [];
    for (let lineIdx = 0; lineIdx < input.length; lineIdx++) {
        const line = input[lineIdx];
        if (line.trim() === '') {
            passportLines.push(passportLine);
            passportLine = '';
        }

        passportLine += ' ' + line;
    }
    if (passportLine !== '') {
        passportLines.push(passportLine);
    }

    const passports = passportLines
        .map(x => {
            const props = x
                .split(' ')
                .filter(y => !!y)
                .map(y => {
                    const prop = y.split(':');
                    return { key: [prop[0]], value: prop[1] };
                })
                .reduce((prev, curr) => Object.assign(prev, { [curr.key]: curr.value }), {});
            return props;
        });

    const rules = {
        byr: (value) => {
            //(Birth Year) - four digits; at least 1920 and at most 2002.
            try {
                const n = parseInt(value);
                return 1920 <= n && n <= 2002;
            }
            catch { return false; }
        },
        iyr: (value) => {
            //(Issue Year) - four digits; at least 2010 and at most 2020.
            try {
                const n = parseInt(value);
                return 2010 <= n && n <= 2020;
            }
            catch { return false; }
        },
        eyr: (value) => {
            //(Expiration Year) - four digits; at least 2020 and at most 2030.
            try {
                const n = parseInt(value);
                return 2020 <= n && n <= 2030;
            }
            catch { return false; }
        },
        hgt: (value) => {
            //(Height) - a number followed by either cm or in:
            //cm, the number must be at least 150 and at most 193.
            //in, the number must be at least 59 and at most 76.
            try {
                const reg = /^([0-9]+)(cm|in)$/;
                const v = reg.exec(value);
                if (!v) {
                    return false;
                }

                if (v[2] == 'cm') {
                    return 150 <= v[1] && v[1] <= 193;
                }

                if (v[2] == 'in') {
                    return 59 <= v[1] && v[1] <= 76;
                }

                return false;
            }
            catch { return false; }
        },
        hcl: (value) => {
            //(Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            try {
                const reg = /^\#[0-9a-f]{6}$/;
                return reg.test(value);
            }
            catch { return false; }
        },
        ecl: (value) => {
            //(Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            try {
                const reg = /^(amb|blu|brn|gry|grn|hzl|oth)$/;
                return reg.test(value);
            }
            catch { return false; }
        },
        pid: (value) => {
            //(Passport ID) - a nine-digit number, including leading zeroes.
            try {
                const reg = /^[0-9]{9}$/;
                return reg.test(value);
            }
            catch { return false; }
        },
        cid: (value) => {
            //(Country ID) - ignored, missing or not.
            return true;
        },
    };

    return passports
        .map(passport => {
            const keys = Object.keys(passport);
            if (keys.length < 7) {
                return 0;
            }

            if (keys.length === 7 && passport.cid) {
                return 0;
            }

            for (let keyIdx = 0; keyIdx < keys.length; keyIdx++) {
                const key = keys[keyIdx];
                const value = passport[key];

                if (!rules[key](value)) {
                    return 0;
                }
            }

            return 1;
        })
        .reduce((prev, curr) => prev + curr, 0);
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
