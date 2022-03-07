import * as Comlink from 'comlink';
import './styles/matmul.css'
import { MatMul } from './workers/matmul';

const WrappedMatMul = Comlink.wrap<typeof MatMul>(new Worker(
  new URL('./workers/matmul.ts', import.meta.url),
  { type: 'module' }
));

const linalg = await new WrappedMatMul();
await linalg.init();
const typedArrays = await new WrappedMatMul();
const arrays = await new WrappedMatMul();

const table: HTMLTableElement = document.querySelector("#size")!;

for (let size = 1e2; size <= 1e3; size += 100) {
  let a = [];
  for (let j = 0; j < size * size; ++j) {
    a.push(Math.random() * 100)
  }
  let b = [];
  for (let j = 0; j < size * size; ++j) {
    b.push(Math.random() * 100)
  }

  const row = document.createElement('tr');
  table.appendChild(row);

  let sizeElement = document.createElement('td');
  sizeElement.innerText = `${size}`
  row.appendChild(sizeElement);

  let la = await linalg.linalg(a, b, size);
  let laElement = document.createElement('td');
  laElement.innerText = `${la.toFixed(3)}`;
  row.appendChild(laElement)

  let ta = await typedArrays.typedArrays(a, b, size);
  let taElement = document.createElement('td');
  taElement.innerText = `${ta.toFixed(3)}`;
  row.appendChild(taElement)

  let ar = await arrays.arrays(a, b, size);
  let arElement = document.createElement('td');
  arElement.innerText = `${ar.toFixed(3)}`;
  row.appendChild(arElement)

  console.log(la, ta, ar);
}
