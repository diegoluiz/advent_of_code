const { performance } = require('perf_hooks');

const fs = require('fs');

const input = fs.readFileSync('d07/input.txt', 'utf8').split('\n');

const quantityRuleRegex = /^(\d+)\s(.+)(bag|bags)/;

function parseRule(rule) {
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    // dotted black bags contain no other bags.

    const [typeName, otherBagsLine] = rule.split(' bags contain ');
    const otherBags = otherBagsLine.split(',').map(otherBag => {
        otherBag = otherBag.trim();
        if (otherBag === 'no other bags.') {
            return null;
        }

        const temp = quantityRuleRegex.exec(otherBag);
        if (temp === null) {
            return null;
        }

        const qnt = parseInt(temp[1]);
        const typeName = temp[2].trim();

        return {
            qnt,
            typeName
        };
    })
        .filter(bag => !!bag);
    return {
        typeName,
        otherBags
    };
}

function parseRules(input) {
    const parsedRules = input.map(parseRule);

    const rules = {};
    parsedRules.forEach(rule => {
        rules[[rule.typeName]] = {
            typeName: rule.typeName,
            otherBags: rule.otherBags
        };
        // rule.type = rules[[rule.typeName]];
    });

    Object.keys(rules).forEach(key => {
        rule = rules[key];

        rule.otherBags = rule.otherBags.map(otherBag => {
            otherBag.type = rules[[otherBag.typeName]];
            if (!otherBag.type.parents) {
                otherBag.type.parents = {};
            }
            otherBag.type.parents[[rule.typeName]] = rule;
            return otherBag;
        });
    });

    return rules;
}

function part1CountBagsRec(bag) {
    if (!bag) {
        return null;
    }

    let bags = [bag.typeName];
    if (bag.parents) {
        const a = Object.keys(bag.parents).map(key => {
            const parentBag = bag.parents[key];
            return part1CountBagsRec(parentBag);
        }).reduce((prev, curr) => prev.concat(curr), []);
        bags = bags.concat(a);
        // return bag.parents.reduce((prev, curr) => prev.concat(curr), bags);
    }

    return bags;
}

function part1() {
    const rules = parseRules(input);

    const bagToFind = 'shiny gold';

    const bag = rules[[bagToFind]];
    const bags = part1CountBagsRec(bag);

    const s = new Set();
    bags.forEach(b => s.add(b));

    return s.size - 1;
}

function part2CountBagsRec(bag) {
    if (!bag) {
        return 0;
    }

    let bags = 1;
    if (bag.otherBags && bag.otherBags.length > 0) {
        const a = Object.keys(bag.otherBags).map(key => {
            const otherBag = bag.otherBags[key];
            const b = otherBag.qnt * part2CountBagsRec(otherBag.type);

            return b;
        });
        bags += a.reduce((prev, curr) => prev + curr, 0);
        // return bag.parents.reduce((prev, curr) => prev.concat(curr), bags);
    }

    return bags;
}

function part2() {
    const rules = parseRules(input);

    const bagToFind = 'shiny gold';

    const bag = rules[[bagToFind]];
    const bags = part2CountBagsRec(bag);
    return bags - 1;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
