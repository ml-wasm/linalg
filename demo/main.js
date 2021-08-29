import init, {
  initThreadPool,
  FloatsMatrix,
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  await initThreadPool(navigator.hardwareConcurrency);

  const response = await fetch('./mat.csv');

  // First try with this
  //const stream = response.body;
  //console.log(stream);
  const a = await FloatsMatrix.newFromCSV(response);
  console.log(a.data);
  // console.log(a.data);

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

