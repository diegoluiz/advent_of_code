const { performance } = require('perf_hooks');

const fs = require('fs');
const input = fs.readFileSync('d23/input.txt', 'utf8');

class Node {
    /**@param {Number} value  */
    constructor(value) {
        this.value = value;

        /** @type {Node} */
        this.next = null;

        /** @type {Node} */
        this.previous = null;
    }
}


function part1() {
    const moves = 100;
    const nums = input.split('').map(x => parseInt(x));
    const head = new Node(nums[0]);
    let curr = head;
    let maxNumber = Math.max(...nums);

    const cupsPool = new Map();
    cupsPool.set(curr.value, curr);
    for (let i = 1; i < nums.length; i++) {
        const next = new Node(nums[i]);
        curr.next = next;
        next.previous = curr;
        curr = next;
        cupsPool.set(curr.value, curr);
    }

    curr.next = head;
    head.previous = curr;

    curr = head;

    for (let i = 0; i < moves; i++) {
        const removed = curr.next;
        cupsPool.delete(removed.value);
        cupsPool.delete(removed.next.value);
        cupsPool.delete(removed.next.next.value);
        curr.next = curr.next.next.next.next;
        curr.next.previous = curr;

        let destinationNumber = curr.value - 1;

        while (destinationNumber >= 0) {
            if (cupsPool.has(destinationNumber)) {
                break;
            }
            destinationNumber--;
        }
        if (destinationNumber < 0) {
            const removedNumbers = new Set([
                removed.value,
                removed.next.value,
                removed.next.next.value,
            ]);
            let tempMax = maxNumber ;
            while (removedNumbers.has(tempMax)) {
                tempMax--;
            }
            destinationNumber = tempMax;
        }
        const destination = cupsPool.get(destinationNumber);
        removed.next.next.next = destination.next;
        removed.previous = destination;

        destination.next.previous = removed;
        destination.next = removed;

        cupsPool.set(removed.value, removed);
        cupsPool.set(removed.next.value, removed.next);
        cupsPool.set(removed.next.next.value, removed.next.next);

        curr = curr.next;
    }

    const first = cupsPool.get(1);
    curr = first;
    let res = '';
    for (let i = 0; i < nums.length - 1; i++) {
        res += curr.next.value;
        curr = curr.next;
    }
    return res;
}

function part2() {
    const moves = 10000000;
    const maxNumber = 1000000;
    const nums = input.split('').map(x => parseInt(x));
    const head = new Node(nums[0]);
    let curr = head;

    let m = Math.max(...nums);
    for (let i = m + 1; i <= maxNumber; i++) {
        nums.push(i);
    }

    const cupsPool = new Map();
    cupsPool.set(curr.value, curr);
    for (let i = 1; i < nums.length; i++) {
        const next = new Node(nums[i]);
        curr.next = next;
        next.previous = curr;
        curr = next;
        cupsPool.set(curr.value, curr);
    }

    curr.next = head;
    head.previous = curr;

    curr = head;

    for (let i = 0; i < moves; i++) {
        const removed = curr.next;
        cupsPool.delete(removed.value);
        cupsPool.delete(removed.next.value);
        cupsPool.delete(removed.next.next.value);
        curr.next = curr.next.next.next.next;
        curr.next.previous = curr;

        let destinationNumber = curr.value - 1;

        while (destinationNumber >= 0) {
            if (cupsPool.has(destinationNumber)) {
                break;
            }
            destinationNumber--;
        }
        if (destinationNumber < 0) {
            const removedNumbers = new Set([
                removed.value,
                removed.next.value,
                removed.next.next.value,
            ]);
            let tempMax = maxNumber ;
            while (removedNumbers.has(tempMax)) {
                tempMax--;
            }
            destinationNumber = tempMax;
        }
        const destination = cupsPool.get(destinationNumber);
        removed.next.next.next = destination.next;
        removed.previous = destination;

        destination.next.previous = removed;
        destination.next = removed;

        cupsPool.set(removed.value, removed);
        cupsPool.set(removed.next.value, removed.next);
        cupsPool.set(removed.next.next.value, removed.next.next);

        curr = curr.next;
    }

    const first = cupsPool.get(1);
    const res = first.next.value * first.next.next.value;
    return res;
}

let start = performance.now();
console.log(`Part 1: [${part1()}]. Time: ${performance.now() - start}`);

start = performance.now();
console.log(`Part 2: [${part2()}]. Time: ${performance.now() - start}`);
