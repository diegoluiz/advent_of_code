const { performance } = require('perf_hooks');
const fs = require('fs');

const dirs = fs.readdirSync('.')
    .filter(x => x.startsWith('d'));

const start = performance.now();
dirs.forEach(d => {
    const appPath = `./${d}/app.js`;
    if (!fs.existsSync(appPath)) {
        return;
    }
    console.log(`Dir: ${d}`);
    require(appPath);
    console.log('');
});


console.log(`Finished in ${performance.now() - start}`);