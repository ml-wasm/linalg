import * as Comlink from 'comlink';
import './style.css'

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

function createVariable(identifier: string, data: Array<Array<number>>): HTMLDivElement {
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

let linalg: any = null;

async function initialize(): Promise<void> {
  linalg = Comlink.wrap(new Worker(
    new URL('./worker.ts', import.meta.url),
    { type: 'module' }
  ));

  const [a, b, c] = await linalg.initialize();
  input.appendChild(createVariable('a', a));
  input.appendChild(createVariable('b', b));
  input.appendChild(createVariable('c', c));

  linalg.test(false);
}

initialize();

code.onkeydown = async key => {
  if (key.code === 'Enter') {
    const result = await linalg.calculate(code.value);
    console.log(result);
    output.childNodes.forEach(c => c.remove());
    output.appendChild(createVariable('result', result));
  }
};


