---
title: FloatsMatrix
---

FloatsMatrix is an two dimensional array or a matrix of 64-bit floats.

The following scripts assume that you have imported the `FloatsMatrix` object
from the package and set up the threads as explained in [getting started](../).


:::note

Some of these have a `R` and `C` variant. `R` variant returns a FloatsVector by
applying the operation row-wise. `C` variant returns a FloatsVector by applying
the operation on each column-wise.

:::

## Constructors Methods

These methods are used to create new `FloatsMatrix`s.

:::note

Random constructors are in the [random section](#random-methods).

:::

```js
// Create a FloatsMatrix from a given JavaScript array
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);
console.log(a.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);

// Both toJSON and data return a JavaScript array representation of the
// FloatsMatrix
console.log(a.toJSON()); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]
console.log(a.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]

// This returns the data and metadata about the FloatsMatrix
console.log(a.toString());
// "[[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]], shape=[2, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2"

// It returns clone of the FloatsMatrix
const b = a.clone();
console.log(b.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]
```

## Utility Methods

Basic getters and setters.

```js
const x = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);

// Get the number of rows in the FloatsMatrix
console.log(x.nrows()); // 2

// Get the number of columns in the FloatsMatrix
console.log(x.ncols()); // 3

// Get the shape of the FloatsMatrix
console.log(x.shape()); // [2, 3]

// Set the given value at the specified index
console.log(x.get([0, 1])); // 0.2

// Get the value at the specified index
x.setR(1, new FloatsVector([0.8, 0.9, 1.0]));
console.log(x.data);
// [[0.1, 0.2, 0.3],
//  [0.9, 0.8, 1]]

// Swap the values at the specified indices
x.swapC(0, 1);
console.log(x.data);
// [[0.1, 0.2, 0.3],
//  [0.9, 0.8, 1]]
```

More complex methods used to manipulate the `FloatsMatrix`.

:::note

Each of these methods has two versions. The "pure" version returns the result of
performing the operation while the "impure" version actually changes the array.

`append -> appended`,
`extend -> extended`,
`insert -> inserted`,
`splice -> spliced`

:::

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);
const b = new FloatsMatrix([
  [0.8, 0.9, 1.0],
  [1.1, 1.2, 1.3],
]);

// Append an element to the FloatsMatrix
console.log(a.appendedR(new FloatsVector([1.4, 1.5, 1.6])).data);
// [[0.1, 0.2, 0.3],
//  [0.5, 0.6, 0.7],
//  [1.4, 1.5, 1.6]]

// Extend the FloatsMatrix with another
console.log(a.extendedC(b).data);
// [[0.1, 0.2, 0.3, 0.8, 0.9, 1.0],
//  [0.5, 0.6, 0.7, 1.1, 1.2, 1.3]]

// Insert the given element at the specified index
console.log(a.insertedR(1, new FloatsVector([1.4, 1.5, 1.6])).data);
// [[0.1, 0.2, 0.3],
//  [1.4, 1.5, 1.6],
//  [0.5, 0.6, 0.7]]

// Removes an element from the specified index
const [spliced, column] = a.splicedC(1);
console.log(spliced.data, column.data);
// [[0.1, 0.3],
//  [0.5, 0.7]]
//
// [0.2, 0.6]
```

## Iteration Methods

These methods allow you to perform element-wise operations on the matrix.

```js
const a = new FloatsMatrix([[0.1, 0.2], [0.3, 0.4]]);

const b = a.map(x => x * 3);
console.log(b.data);
// [[0.30000000000000004, 0.6000000000000001],
//  [0.8999999999999999, 1.2000000000000002]]

a.forEach(x => console.log(x));
// 0.1
// 0.2
// 0.3
// 0.4

a.transform(x => x * x);
console.log(a.data);
// [[0.010000000000000002, 0.04000000000000001],
//  [0.09, 0.16000000000000003]],
```

## Math Methods

Methods to perform simple mathematical operations on the array.

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);
const b = new FloatsMatrix([
  [0.8, 0.9, 1.0],
  [1.1, 1.2, 1.6],
]);

// Perform element-wise addition of two FloatsMatrices
console.log(a.add(b).data);
// [[0.9, 1.1, 1.3],
//  [1.6, 1.7999999999999998, 2.3]]

// Perform element-wise subtraction of two FloatsMatrices
console.log(a.sub(b).data);
// [[0.9, 1.1, 1.3],
//  [1.6, 1.7999999999999998, 2.3]]

// Perform element-wise multiplication of two FloatsMatrices
console.log(a.mul(b).data);
// [[0.08000000000000002, 0.18000000000000002, 0.3],
//  [0.55, 0.72, 1.1199999999999999]]

// Perform element-wise division of two FloatsMatrices
console.log(b.div(a).data);
// [[8, 4.5, 3.3333333333333335],
//  [2.2, 2, 2.285714285714286]]

// Transposes the FloatsMatrix. That is it exchanges the rows and columns.
// a.tranposed() will return the result of performing the operations
b.transpose();
console.log(b.data);
// [[0.8, 1.1],
//  [0.9, 1.2],
//  [1, 1.6]]

// Performs dot product of the two FloatsMatrices
console.log(a.dot(b).data);
// [[0.56, 0.83],
//  [1.6400000000000001, 2.3899999999999997]]

// Return the addition or product of the FloatsMatrices
console.log(a.sum()); // 2.4000000000000004
console.log(b.product()); // 1.5206400000000002

// Efficiently perform in-place element-wise scaled addition of two FloatsMatrices
a.scaledAdd(2, b.transposed());
console.log(a.data);
// [[1.7000000000000002, 2, 2.3],
//  [2.7, 3, 3.9000000000000004]]
```

## Statistical Methods

Methods to perform basic statistical operations.

:::note

All of these methods has a `R`, `C` and normal variant. `R` variant returns a
FloatsVector by applying the operation on each row. `C` variant returns a
FloatsVector by applying the operation on each column.

:::

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.4, 0.5, 0.6],
]);

// Return the minimum element in the array
console.log(a.min()); // 0.1

// Return the minimum element in each row
console.log(a.maxR().data); // [0.3, 0.6]

// Returns the mean of each column
console.log(a.meanC().data); // [0.25, 0.35, 0.44999999999999996]

// Return the standard deviation in each row
console.log(a.stdR(0).data); // [0.0816496580927726, 0.08164965809277258]

// Return the variance of the array
console.log(a.varC(1).data); // [0.04500000000000001, 0.045000000000000005, 0.045000000000000005]
```

## Random Methods

```js
const a = FloatsMatrix.newWithRandom([3, 2]);
console.log(a.data);
// [[0.40888454798641727, 0.3739993633147044],
//  [0.08152247640640187, 0.8203409920415033],
//  [0.29456711455073004, 0.6857528913804324]],

const b = FloatsMatrix.newWithRandomBeta([3, 2], 2, 5);
console.log(b.data);
// [[0.4352666050451828, 0.2452383631782982  ],
//  [0.05846582502047908, 0.4333062397167741 ],
//  [0.04941459693399252, 0.49269775774198726]]

const c = FloatsMatrix.newWithRandomCauchy([3, 2], 2, 5);
console.log(c.data);
// [[-11.385961920418827, -17.15161173318022],
//  [0.1024638854925195 , 6.279745583099606 ],
//  [3.3623184985082037 , 3.885489673921221 ]]

const d = FloatsMatrix.newWithRandomChiSquared([3, 2], 11);
console.log(d.data);
// [[10.277459169762322, 35.62020788455194 ],
//  [4.284340214042332 , 24.45187876986452 ],
//  [7.931668582420599 , 16.316163514969134]]

const e = FloatsMatrix.newWithRandomExp([3, 2], 10);
console.log(e.data);
// [[0.005281103503605415 , 0.015056308235273966],
//  [0.27406523498818836  , 0.09097148317752426 ],
//  [0.006186609231888619 , 0.14851041454423092 ]]

const f = FloatsMatrix.newWithRandomExp1([3, 2]);
console.log(f.data);
// [[2.9591898182632073 , 0.8112187545007865],
//  [0.47473323680065416, 1.113445155678973 ],
//  [0.49151564307595746, 1.1401397947664906]]

const g = FloatsMatrix.newWithRandomFisher([3, 2], 2, 32);
console.log(g.data);
// [[6.035022100298939 , 0.11195752275263829],
//  [0.7880226226513286, 0.6831958349556548 ],
//  [1.6891867144690675, 1.9251576486424293 ]]

const h = FloatsMatrix.newWithRandomGamma([3, 2], 2, 5);
console.log(h.data);
// [[7.297140827545098 , 15.075130704066002],
//  [5.205631783926043 , 6.907301503082457 ],
//  [3.9806281284745024, 2.5184095054934668]]

const i = FloatsMatrix.newWithRandomInverseGaussian([3, 2], 2, 5);
console.log(i.data);
// [[1.2362125958158847, 2.7094177472381786],
//  [3.257640695896601 , 1.7968259845332966],
//  [6.733285552808544 , 0.7546923920545539]]

const j = FloatsMatrix.newWithRandomLogNormal([3, 2], 2, 3);
console.log(j.data);
// [[1.5944633208973324 , 0.5804460356679529],
//  [56.452828478914235 , 259.24032587609355],
//  [0.18045171237969937, 2.314717853874176 ]]

const k = FloatsMatrix.newWithRandomNormal([3, 2], 2, 3);
console.log(k.data);
// [[-4.937582097353862, -3.304836741396361],
//  [0.5065334060859159, 4.684446356297737 ],
//  [1.7827328981046664, 1.0296375546818184]]

const l = FloatsMatrix.newWithRandomNormalInverseGaussian([3, 2], 3, 2);
console.log(l.data);
// [[-0.31152530286956825, 0.639648853420095 ],
//  [0.5919188019185757  , 1.949471648391123 ],
//  [0.4631301018401633  , 1.2970831389679616]]

const m = FloatsMatrix.newWithRandomOpen01([3, 2]);
console.log(m.data);
// [[0.39278753839723257, 0.6371308131278645],
//  [0.4658275442670957 , 0.6107343971136397],
//  [0.9566538341934372 , 0.6313048135752762]]

const n = FloatsMatrix.newWithRandomOpenClosed01([3, 2]);
console.log(n.data);
// [[0.7442221338996284 , 0.14949176424893484],
//  [0.11589327743372535, 0.8767809717607971 ],
//  [0.9017807183733559 , 0.20821136682264907]]

const o = FloatsMatrix.newWithRandomPERT([3, 2], 0, 5, 2.5);
console.log(o.data);
// [[3.5855992452910885, 1.298769153985166 ],
//  [2.6433936456914715, 4.057706593468385 ],
//  [3.7000687237710035, 3.9289862203349935]]

const p = FloatsMatrix.newWithRandomPERTwithShape([3, 2], 0, 5, 2.5, 3);
console.log(p.data);
// [[4.113333526166986 , 1.5249031843815544],
//  [3.0846709909462557, 1.7871186584755294],
//  [1.6376575755162703, 4.096643354436632 ]]

const r = FloatsMatrix.newWithRandomPareto([3, 2], 1, 2);
console.log(r.data);
// [[1.2218668439832818, 1.6203296802965663],
//  [2.4633071191032023, 3.402201249428801 ],
//  [1.0272698414424728, 1.2468035617869209]]

const s = FloatsMatrix.newWithRandomPoisson([3, 2], 2.0);
console.log(s.data);
// [[1.0, 1.0],
//  [1.0, 1.0],
//  [2.0, 0.0]]

const t = FloatsMatrix.newWithRandomStandardNormal([3, 2]);
console.log(t.data);
// [[-0.03480684481808905, -0.2367400510322934 ],
//  [1.3014550243997869  , 1.1395929103674205  ],
//  [0.2940861674794777  , -0.06811041348658564]]

const u = FloatsMatrix.newWithRandomStudentT([3, 2], 11);
console.log(u.data);
// [[-0.9730798806120124, -2.4774924587564855],
//  [-0.8038916959915411, 0.38853519831079814],
//  [1.48599116493284   , 0.19963185132348066]]

const v = FloatsMatrix.newWithRandomTriangular([3, 2], 0, 5, 2.5);
console.log(v.data);
// [[0.24923070839180786, 2.0092007287696343],
//  [3.412657810267511  , 3.9658729074861685],
//  [1.6378056427234886 , 3.390844513218333 ]]

const w = FloatsMatrix.newWithRandomUniform([3, 2], -5, 5);
console.log(w.data);
// [[-3.6360495078042154 , 4.960282899472354  ],
//  [-0.45696304892605966, -3.913525369070674 ],
//  [-2.271588129876956  , 0.07717747041089851]]

const x = FloatsMatrix.newWithRandomWeibull([3, 2], 1, 10);
console.log(x.data);
// [[1.0444881410425038, 0.9303758536841533],
//  [0.8812800857921872, 1.1002321732214724],
//  [1.1453184784018968, 0.9719495544740979]]

const aa = a.sampleR(2);
console.log(aa.data);
// [[0.4661858441911314, 0.7320500923756094],
//  [0.6274918901836912, 0.6661593918373582]]
```
