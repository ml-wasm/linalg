import * as Comlink from 'comlink';
import './style.css'
import { Array1d, Array2d } from './utils/types';

import { Linalg } from './worker';

const input = document.getElementById("input") as HTMLDivElement;
const code = document.getElementById("code") as HTMLInputElement;
const output = document.getElementById("output") as HTMLDivElement;

function createMatrix(data: Array<Array<number>>): HTMLDivElement {
  const matrix = document.createElement("div");
  matrix.classList.add("matrix");
  matrix.style.gridTemplateRows = `repeat(${data.length}, 1fr)`;
  matrix.style.gridTemplateColumns = `repeat(${data[0].length}, 1fr)`;

  data.forEach(row => row.forEach(ele => {
    const cell = document.createElement("span");
    cell.classList.add("cell");
    cell.innerText = ele.toFixed(2);
    matrix.appendChild(cell);
  }));

  return matrix;
}

function createVariable(identifier: string, data: Array2d): HTMLDivElement {
  const variable = document.createElement("div");
  variable.classList.add("variable");

  const name = document.createElement("span");
  name.innerText = identifier;
  variable.appendChild(name);

  const equalToSymbol = document.createElement("span");
  equalToSymbol.innerText = "=";
  variable.appendChild(equalToSymbol);

  const matrixDiv = createMatrix(data);
  variable.appendChild(matrixDiv);

  return variable;
}

function populateDiv(div: HTMLDivElement, matrices: Array<[string, Array2d]>) {
  console.log(matrices);

  for (let [identifier, data] of matrices) {
    div.appendChild(createVariable(identifier, data));
  }
}

const aSize = [2, 3];
const bSize = [2, 3];
const cSize = [3, 2];
// let a: Array2d | null = null, b: Array2d | null = null, c: Array2d | null = null;

async function initialize(): Promise<void> {
  const linalg: Comlink.Remote<Linalg> = Comlink.wrap(new Worker(
    new URL('./worker.ts', import.meta.url),
    { type: 'module' }
  ));

  await linalg.initialize();
  const [a, b, c] = await linalg.createDemo(aSize, bSize, cSize);
  populateDiv(input, [['a', a], ['b', b], ['c', c]])

  code.onkeydown = async key => {
    if (key.code === 'Enter') {
      const { result, changed } = await linalg.calculate(code.value, a, b, c);

      while (output.firstChild) { output.removeChild(output.firstChild) }

      if (result) {
        populateDiv(output, [['result', result]]);
      } else {
        populateDiv(output, [['a', changed[0]], ['b', changed[1]], ['c', changed[2]]]);
      }
    }
  };
}

initialize();


