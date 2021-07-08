import init, { FloatsVector, FloatsMatrix } from '../../../pkg/linalg.js';
import { createDisplayDiv } from './display.js';

(async () => {
  await init();
  const a = new FloatsMatrix([
    [1, 2, 3],
    [4, 5, 6],
  ]);
  const b = new FloatsMatrix([
    [7, 8, 9],
    [10, 11, 12],
  ]);

  const start = document.querySelector('section.start');
  start.appendChild(createDisplayDiv('a', a));
  start.appendChild(createDisplayDiv('b', b));

  const code = document.getElementById('code');
  code.addEventListener('keypress', e => {
    if (e.key === 'Enter') {
      run(code.value);
    }
  });
})();

const run = text => {
  const a = new FloatsMatrix([
    [1, 2, 3],
    [4, 5, 6],
  ]);
  const b = new FloatsMatrix([
    [7, 8, 9],
    [10, 11, 12],
  ]);

  const func = new Function('a', 'b', 'Floats1d', 'Floats2d', `return ${text}`);
  const ret = func(a, b, FloatsVector, FloatsMatrix);

  const end = document.querySelector('section.end');
  while (end.firstChild) {
    end.firstChild.remove();
  }

  if (ret) {
    end.appendChild(createDisplayDiv('return', ret));
  } else {
    end.appendChild(createDisplayDiv('a', a));
    end.appendChild(createDisplayDiv('b', b));
  }
};
