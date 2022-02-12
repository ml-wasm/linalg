import * as Comlink from 'comlink';
import './style.css'
import { Array2d } from './utils/types';
import { ReplDemo } from './workers/repl-demo-worker';

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

function populateDiv(div: HTMLDivElement, arraysData: Record<string, Array2d>) {
  for (let [identifier, data] of Object.entries(arraysData)) {
    div.appendChild(createVariable(identifier, data));
  }
}

function clearDiv(div: HTMLDivElement) {
  while (div.firstChild) { div.removeChild(div.firstChild) }
}

const arraySizes = {
  'a': [2, 3],
  'b': [2, 3],
  'c': [3, 2],
}

const WrappedRepl = Comlink.wrap<typeof ReplDemo>(new Worker(
  new URL('./workers/repl-demo-worker.ts', import.meta.url),
  { type: 'module' }
));

const demo = await new WrappedRepl();
await demo.init();

await demo.createArrays(arraySizes);
const arraysData = await demo.getArraysData();
populateDiv(input, arraysData);

code.onkeydown = async key => {
  if (key.code === 'Enter') {
    const toShow = await demo.performOperation(code.value);

    clearDiv(output);
    populateDiv(output, toShow);
  }
};


