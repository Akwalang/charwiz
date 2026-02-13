const fs = require('fs');
const path = require('path');

const cp = require('child_process');

const files = [
  './data/art.cjs',
  './data/ui.cjs',
];

let time = +new Date();

function myFunction() {
  if (time > +new Date() - 2000) return;

  time = +new Date();

  const cmd = `node ${path.resolve(__dirname, './run.cjs')}`;

  console.log('Running...');
  cp.exec(cmd, (err, stdout, stderr) => {
    if (err) {
      console.error(`- Error: ${err}`);
      return;
    }
    if (stderr) {
      console.error(`- Warn: ${stderr}`);
      return;
    }
    console.log(`${stdout}`);
  });
  console.log(`Done! ${+new Date() - time}ms`);
}

const run = (() => {
  files.forEach((file) => {
    file = path.join(__dirname, file);

    fs.watch(file, (eventType, filename) => {
      eventType === 'change' && myFunction();
    });
  });
});

run();
