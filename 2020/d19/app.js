const { performance } = require('perf_hooks');

const _ = require('lodash');
const fs = require('fs');

const input = fs.readFileSync('d19/input.txt', 'utf8').split('\n');

function part1() {
    const rules = {};
    const messages = [];

    input.forEach(line => {
        if (line.indexOf(':') >= 0) {
            let [id, ruleContent] = line.split(':');
            let subRules = ruleContent.split('|').map(x => x.split(' ').filter(x => x));
            rules[[id]] = { subRules };
        } else {
            messages.push(line);
        }
    });

    const ruleCache = {};
    const parseRule = (rules, ruleId) => {
        if (ruleCache[[ruleId]]) {
            return ruleCache[[ruleId]];
        }

        const rule = rules[[ruleId]];
        if (rule.subRules.length == 1 && rule.subRules[0][0].indexOf('"') >= 0) {
            rule.expectedStrings = [rule.subRules[0][0].substring(1, 2)];
            return rule.expectedStrings;
        }

        rule.expectedStrings = _.flatMapDeep(rule.subRules.map(x => {
            return _.chain(x)
                .map(y => parseRule(rules, y))
                .reduce((p, c) => _.flattenDeep(p.map(xa => c.map(xb => xa + xb))), [''])
                .flattenDeep()
                .value();
        }));

        ruleCache[[ruleId]] = rule.expectedStrings;

        return rule.expectedStrings;
    };

    const expected = new Set(parseRule(rules, '0'));
    return messages.filter((message) => expected.has(message)).length;
}

function part2() {
    const rules = {};
    const messages = [];

    input.forEach(line => {
        if (line.indexOf(':') >= 0) {
            let [id, ruleContent] = line.split(':');
            let subRules = ruleContent.split('|').map(x => x.split(' ').filter(x => x));
            rules[[id]] = { subRules };
        } else {
            messages.push(line);
        }
    });

    const ruleCache = {};
    const parseRule = (rules, ruleId) => {
        if (ruleCache[[ruleId]]) {
            return ruleCache[[ruleId]];
        }

        const rule = rules[[ruleId]];
        if (rule.subRules.length == 1 && rule.subRules[0][0].indexOf('"') >= 0) {
            rule.expectedStrings = [rule.subRules[0][0].substring(1, 2)];
            return rule.expectedStrings;
        }

        rule.expectedStrings = _.flatMapDeep(rule.subRules.map(x => {
            return _.chain(x)
                .map(y => parseRule(rules, y))
                .reduce((p, c) => _.flattenDeep(p.map(xa => c.map(xb => xa + xb))), [''])
                .flattenDeep()
                .value();
        }));

        ruleCache[[ruleId]] = rule.expectedStrings;

        return rule.expectedStrings;
    };

    const rule42Expected = new Set(parseRule(rules, '42'));
    const rule31Expected = new Set(parseRule(rules, '31'));

    const customFilter = (message) => {
        const blockSize = rule42Expected.values().next().value.length;
        let tempMessage = message;
        let foundCount = 0;
        let found = false;
        do {
            let end = tempMessage.substring(tempMessage.length - blockSize);
            found = rule31Expected.has(end);
            if (found) {
                tempMessage = tempMessage.substring(0, tempMessage.length - blockSize);
                foundCount++;
            }
        } while (found);

        if (foundCount < 1) {
            return false;
        }

        for (let i = 0; i < foundCount; i++) {
            let end = tempMessage.substring(tempMessage.length - blockSize);
            found = rule31Expected.has(end);
            if (found) {
                return false;
            }
            tempMessage = tempMessage.substring(0, tempMessage.length - blockSize);
        }

        foundCount = 0;
        do {
            let start = tempMessage.substring(0, blockSize);
            found = rule42Expected.has(start);
            if (found) {
                tempMessage = tempMessage.substring(blockSize);
                foundCount++;
            }
        } while (found);

        if (foundCount < 1 || tempMessage !== '') {
            return false;
        }

        return true;
    };

    const ret = messages.filter(customFilter);

    return ret.length;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
