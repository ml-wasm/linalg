const createNumDiv = (id, num) => {
  const numDiv = document.createElement('div');
  numDiv.classList.add('num');
  numDiv.innerText = `${id} = ${num.toFixed(2)}`;

  return numDiv;
};

const createOneDiv = (id, one) => {
  const oneDiv = document.createElement('div');
  oneDiv.classList.add('one', 'mat');

  const tagDiv = document.createElement('div');
  tagDiv.innerText = `${id} = `;
  oneDiv.appendChild(tagDiv);

  const dataDiv = document.createElement('div');
  dataDiv.classList.add('data');
  const [cols] = one.shape();
  dataDiv.style.gridTemplateRows = '1fr';
  dataDiv.style.gridTemplateColumns = `repeat(${cols}, 1fr)`;
  oneDiv.appendChild(dataDiv);

  one.data.forEach(element => {
    const cellDiv = document.createElement('div');
    cellDiv.classList.add('cell');
    dataDiv.appendChild(cellDiv);

    cellDiv.innerText = element.toFixed(2);
  });

  return oneDiv;
};

const createTwoDiv = (id, two) => {
  const twoDiv = document.createElement('div');
  twoDiv.classList.add('one', 'mat');

  const tagDiv = document.createElement('div');
  tagDiv.innerText = `${id} = `;
  twoDiv.appendChild(tagDiv);

  const dataDiv = document.createElement('div');
  dataDiv.classList.add('data');
  const [rows, cols] = two.shape();
  dataDiv.style.gridTemplateRows = `repeat(${rows}, 1fr)`;
  dataDiv.style.gridTemplateColumns = `repeat(${cols}, 1fr)`;
  twoDiv.appendChild(dataDiv);

  two.data.forEach(row => {
    row.forEach(element => {
      const cellDiv = document.createElement('div');
      cellDiv.classList.add('cell');
      dataDiv.appendChild(cellDiv);

      cellDiv.innerText = element.toFixed(2);
    });
  });

  return twoDiv;
};

export const createDisplayDiv = (id, object) => {
  if (typeof object === 'number') {
    return createNumDiv(id, object);
  } else if (typeof object === 'object') {
    if (object.shape().length == 1) {
      return createOneDiv(id, object);
    } else if (object.shape().length == 2) {
      return createTwoDiv(id, object);
    }
  }
};
