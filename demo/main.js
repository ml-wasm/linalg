import init, {
  initThreadPool,
  FloatsMatrix,
  FloatsVector,
  IntegersMatrix
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  //await initThreadPool(navigator.hardwareConcurrency);

  //const response = await fetch('./mat.csv');

  // First try with this
  //const stream = response.body;
  //console.log(stream);
  //const a = await FloatsMatrix.newFromCSV(response);
  //console.log(a.data);
  // console.log(a.data);

  //const vec_file = await fetch('./vec.csv');
  //let vec = await FloatsVector.newFromCSV(vec_file);
  //console.log(vec.data);
  
  const mat = IntegersMatrix.newFromTypedArray(new Int32Array([1.1, 2.2, 4.2, 6.2, 2.4, 8.6]), 2, 3);
  console.log(mat.data);
  // If this doesn't work then just pass the entire text to the function
   //const text = await response.text();
   //console.log(text);
   //const a = FloatsMatrix.newFromCSV(text);
  // console.log(a.data);
})();

// Ignore this for now
// const btn = document.getElementById("file-picker");
// btn.addEventListener("click", async () => {
//   console.log("Hello");
//   const [fileHandle] = await window.showOpenFilePicker();
//   const file = await fileHandle.getFile();
// 
//   // const a = FloatsMatrix.newFromCSV(file.stream());
//   // const b = FloatsVector.newFromCSV(file.text());
// });

