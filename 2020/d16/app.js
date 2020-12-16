const { performance } = require('perf_hooks');

const fs = require('fs');
const _ = require('lodash');
const { group } = require('console');
const input = fs.readFileSync('d16/input.txt', 'utf8');

function part1() {
    const inputGroups = input.split('\n\n');

    const rules = _.flattenDeep(inputGroups[0].split('\n')
        .map(rule => rule
            .split(' ')
            .filter(y => y.indexOf('-') >= 0)
            .map(x => {
                const t = x.split('-');
                return {
                    from: parseInt(t[0]),
                    to: parseInt(t[1]),
                };
            })
        ));
    const myTicket = inputGroups[1].split('\n')[1].split(',').map(x => parseInt(x));
    const nearbyTickets = inputGroups[2].split('\n').splice(1).map(x => x.split(',').map(y => parseInt(y)));

    let errorRate = 0;
    nearbyTickets.forEach(ticket => {
        ticket.forEach(number => {
            let good = false;
            rules.forEach(rule => {
                if (rule.from <= number && number <= rule.to) {
                    good = true;
                }
            });
            if (!good) {
                errorRate += number;
            }
        });
    });

    return errorRate;
}

function part2() {
    const inputGroups = input.split('\n\n');

    const ruleNames = new Set();
    const validationRules = _.flattenDeep(inputGroups[0].split('\n')
        .map(rule => rule
            .split(' ')
            .filter(y => y.indexOf('-') >= 0)
            .map(x => {
                ruleNames.add(rule.split(':')[0]);
                const t = x.split('-');
                return {
                    from: parseInt(t[0]),
                    to: parseInt(t[1]),
                    name: rule.split(':')[0]
                };
            })
        ));
    const myTicket = inputGroups[1].split('\n')[1].split(',').map(x => parseInt(x));
    const validNearbyTickets = inputGroups[2]
        .split('\n')
        .splice(1)
        .map(x => x.split(',').map(y => parseInt(y)))
        .filter(ticket => {
            let valid = true;
            ticket.forEach(number => {
                let good = false;
                validationRules.forEach(rule => {
                    if (rule.from <= number && number <= rule.to) {
                        good = true;
                    }
                });
                if (!good) {
                    valid = false;
                }
            });
            return valid;
        });

    const rulesPositionsSets = {};
    for (let i = 0; i < validNearbyTickets[0].length; i++) {
        rulesPositionsSets[i] = new Set(ruleNames);
    }

    const groupedRules = _.chain(validationRules)
        .groupBy(x => x.name)
        .value();

    validNearbyTickets.forEach(ticket => {
        for (let i = 0; i < ticket.length; i++) {
            const number = ticket[i];
            _.map(groupedRules, (v, k) => {
                let good = false;
                v.forEach(rule => {
                    if (rule.from <= number && number <= rule.to) {
                        good = true;
                    }
                });
                if (!good) {
                    rulesPositionsSets[i].delete(k);
                }
            });
        }
    });

    const used = new Set();
    const departureRules = _.chain(rulesPositionsSets)
    .map((v, k) => ({ idx: k, s: v, l: v.size }))
    .sortBy(x => x.l)
    .map(x => {
        used.forEach(y => x.s.delete(y));
        x.s.forEach(y => used.add(y));
        return x;
    })
    .map(x => {
        return {
            idx: x.idx,
            name: x.s.values().next().value
        };
    })
    .filter(x => x.name.startsWith('departure'))
    .value();
    
    const res = departureRules.reduce((p,c) => p * myTicket[c.idx], 1);
    
    return res;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
