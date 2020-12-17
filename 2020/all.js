const { performance } = require('perf_hooks');
const fs = require('fs');

const dirs = fs.readdirSync('.')
    .filter(x => x.startsWith('d'));

const start = performance.now();
dirs.forEach(d => {
    console.log(`Dir: ${d}`);
    require(`./${d}/app.js`);
    console.log('');
});


console.log(`Finished in ${performance.now() - start}`);