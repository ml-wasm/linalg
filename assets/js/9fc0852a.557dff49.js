(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[710],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return d},kt:function(){return h}});var a=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,o=function(e,t){if(null==e)return{};var n,a,o={},r=Object.keys(e);for(a=0;a<r.length;a++)n=r[a],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(a=0;a<r.length;a++)n=r[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var l=a.createContext({}),c=function(e){var t=a.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},d=function(e){var t=c(e.components);return a.createElement(l.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,o=e.mdxType,r=e.originalType,l=e.parentName,d=s(e,["components","mdxType","originalType","parentName"]),m=c(n),h=o,u=m["".concat(l,".").concat(h)]||m[h]||p[h]||r;return n?a.createElement(u,i(i({ref:t},d),{},{components:n})):a.createElement(u,i({ref:t},d))}));function h(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var r=n.length,i=new Array(r);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s.mdxType="string"==typeof e?e:o,i[1]=s;for(var c=2;c<r;c++)i[c]=n[c];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},2132:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return s},contentTitle:function(){return l},metadata:function(){return c},toc:function(){return d},default:function(){return m}});var a=n(2122),o=n(9756),r=(n(7294),n(3905)),i=["components"],s={title:"FloatsMatrix"},l=void 0,c={unversionedId:"Matrices/FloatsMatrix",id:"Matrices/FloatsMatrix",isDocsHomePage:!1,title:"FloatsMatrix",description:"FloatsMatrix is an two dimensional array or a matrix of 64-bit floats.",source:"@site/docs/03-Matrices/03-FloatsMatrix.md",sourceDirName:"03-Matrices",slug:"/Matrices/FloatsMatrix",permalink:"/linalg/Matrices/FloatsMatrix",editUrl:"https://github.com/wasml/linalg/edit/master/docs/docs/03-Matrices/03-FloatsMatrix.md",version:"current",sidebarPosition:3,frontMatter:{title:"FloatsMatrix"},sidebar:"tutorialSidebar",previous:{title:"IntegersMatrix",permalink:"/linalg/Matrices/IntegersMatrix"},next:{title:"StringsMatrix",permalink:"/linalg/Matrices/StringsMatrix"}},d=[{value:"Constructors Methods",id:"constructors-methods",children:[]},{value:"Interop Methods",id:"interop-methods",children:[]},{value:"Utility Methods",id:"utility-methods",children:[]},{value:"Iteration Methods",id:"iteration-methods",children:[]},{value:"Math Methods",id:"math-methods",children:[]},{value:"Statistical Methods",id:"statistical-methods",children:[]}],p={toc:d};function m(e){var t=e.components,n=(0,o.Z)(e,i);return(0,r.kt)("wrapper",(0,a.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"FloatsMatrix is an two dimensional array or a matrix of 64-bit floats."),(0,r.kt)("p",null,"The following scripts assume that you have imported the ",(0,r.kt)("inlineCode",{parentName:"p"},"FloatsMatrix")," object\nfrom the package and set up the threads as explained in ",(0,r.kt)("a",{parentName:"p",href:"../"},"getting started"),"."),(0,r.kt)("h2",{id:"constructors-methods"},"Constructors Methods"),(0,r.kt)("p",null,"These methods are used to create new ",(0,r.kt)("inlineCode",{parentName:"p"},"FloatsMatrix"),"s."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"// Create a FloatsMatrix from a given JavaScript array\nconst a = new FloatsMatrix([\n  [0.1, 0.2, 0.3],\n  [0.5, 0.6, 0.7],\n]);\nconsole.log(a.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]\n")),(0,r.kt)("h2",{id:"interop-methods"},"Interop Methods"),(0,r.kt)("p",null,"Some handy methods to work with the array."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},'const a = new FloatsMatrix([\n  [0.1, 0.2, 0.3],\n  [0.5, 0.6, 0.7],\n]);\n\n// Both toJSON and data return a JavaScript array representation of the\n// FloatsMatrix\nconsole.log(a.toJSON()); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]\nconsole.log(a.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]\n\n// This returns the data and metadata about the FloatsMatrix\nconsole.log(a.toString());\n// "[[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]], shape=[2, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2"\n\n// It returns clone of the FloatsMatrix\nconst b = a.clone();\nconsole.log(b.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]\n')),(0,r.kt)("h2",{id:"utility-methods"},"Utility Methods"),(0,r.kt)("p",null,"Basic getters and setters."),(0,r.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,r.kt)("div",{parentName:"div",className:"admonition-heading"},(0,r.kt)("h5",{parentName:"div"},(0,r.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,r.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,r.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,r.kt)("div",{parentName:"div",className:"admonition-content"},(0,r.kt)("p",{parentName:"div"},(0,r.kt)("inlineCode",{parentName:"p"},"get"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"set")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"swap")," have a ",(0,r.kt)("inlineCode",{parentName:"p"},"R"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"C")," and a normal variant. ",(0,r.kt)("inlineCode",{parentName:"p"},"R")," variant\napplies the operation to the specified row(s). ",(0,r.kt)("inlineCode",{parentName:"p"},"C")," variant applies the operation\nto the specified column(s)."))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const x = new FloatsMatrix([\n  [0.1, 0.2, 0.3],\n  [0.5, 0.6, 0.7],\n]);\n\n// Get the number of rows in the FloatsMatrix\nconsole.log(x.nrows()); // 2\n\n// Get the number of columns in the FloatsMatrix\nconsole.log(x.ncols()); // 3\n\n// Get the shape of the FloatsMatrix\nconsole.log(x.shape()); // [2, 3]\n\n// Set the given value at the specified index\nconsole.log(x.get([0, 1])); // 0.2\n\n// Get the value at the specified index\nx.setR(1, new FloatsVector([0.8, 0.9, 1.0]));\nconsole.log(x.data);\n// [[0.1, 0.2, 0.3],\n//  [0.9, 0.8, 1]]\n\n// Swap the values at the specified indices\nx.swapC(0, 1);\nconsole.log(x.data);\n// [[0.1, 0.2, 0.3],\n//  [0.9, 0.8, 1]]\n")),(0,r.kt)("p",null,"More complex methods used to manipulate the ",(0,r.kt)("inlineCode",{parentName:"p"},"FloatsMatrix"),"."),(0,r.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,r.kt)("div",{parentName:"div",className:"admonition-heading"},(0,r.kt)("h5",{parentName:"div"},(0,r.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,r.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,r.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,r.kt)("div",{parentName:"div",className:"admonition-content"},(0,r.kt)("p",{parentName:"div"},'Each of these methods has two versions. The "pure" version returns the result of\nperforming the operation while the "impure" version actually changes the array.'),(0,r.kt)("p",{parentName:"div"},(0,r.kt)("inlineCode",{parentName:"p"},"append -> appended"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"extend -> extended"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"insert -> inserted"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"splice -> spliced")))),(0,r.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,r.kt)("div",{parentName:"div",className:"admonition-heading"},(0,r.kt)("h5",{parentName:"div"},(0,r.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,r.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,r.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,r.kt)("div",{parentName:"div",className:"admonition-content"},(0,r.kt)("p",{parentName:"div"},"Each of these has a ",(0,r.kt)("inlineCode",{parentName:"p"},"R")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"C")," variant. ",(0,r.kt)("inlineCode",{parentName:"p"},"R")," variant returns a FloatsVector by\napplying the operation row-wise. ",(0,r.kt)("inlineCode",{parentName:"p"},"C")," variant returns a FloatsVector by applying\nthe operation on each column-wise."))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsMatrix([\n  [0.1, 0.2, 0.3],\n  [0.5, 0.6, 0.7],\n]);\nconst b = new FloatsMatrix([\n  [0.8, 0.9, 1.0],\n  [1.1, 1.2, 1.3],\n]);\n\n// Append an element to the FloatsMatrix\nconsole.log(a.appendedR(new FloatsVector([1.4, 1.5, 1.6])).data);\n// [[0.1, 0.2, 0.3],\n//  [0.5, 0.6, 0.7],\n//  [1.4, 1.5, 1.6]]\n\n// Extend the FloatsMatrix with another\nconsole.log(a.extendedC(b).data);\n// [[0.1, 0.2, 0.3, 0.8, 0.9, 1.0],\n//  [0.5, 0.6, 0.7, 1.1, 1.2, 1.3]]\n\n// Insert the given element at the specified index\nconsole.log(a.insertedR(1, new FloatsVector([1.4, 1.5, 1.6])).data);\n// [[0.1, 0.2, 0.3],\n//  [1.4, 1.5, 1.6],\n//  [0.5, 0.6, 0.7]]\n\n// Removes an element from the specified index\nconst [spliced, column] = a.splicedC(1);\nconsole.log(spliced.data, column.data);\n// [[0.1, 0.3],\n//  [0.5, 0.7]]\n//\n// [0.2, 0.6]\n")),(0,r.kt)("h2",{id:"iteration-methods"},"Iteration Methods"),(0,r.kt)("p",null,"These methods allow you to perform element-wise operations on the matrix."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsMatrix([[0.1, 0.2], [0.3, 0.4]]);\n\nconst b = a.map(x => x * 3);\nconsole.log(b.data);\n// [[0.30000000000000004, 0.6000000000000001],\n//  [0.8999999999999999, 1.2000000000000002]]\n\na.forEach(x => console.log(x));\n// 0.1\n// 0.2\n// 0.3\n// 0.4\n\na.transform(x => x * x);\nconsole.log(a.data);\n// [[0.010000000000000002, 0.04000000000000001],\n//  [0.09, 0.16000000000000003]],\n")),(0,r.kt)("h2",{id:"math-methods"},"Math Methods"),(0,r.kt)("p",null,"Methods to perform simple mathematical operations on the array."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsMatrix([\n  [0.1, 0.2, 0.3],\n  [0.5, 0.6, 0.7],\n]);\nconst b = new FloatsMatrix([\n  [0.8, 0.9, 1.0],\n  [1.1, 1.2, 1.6],\n]);\n\n// Perform element-wise addition of two FloatsMatrices\nconsole.log(a.add(b).data);\n// [[0.9, 1.1, 1.3],\n//  [1.6, 1.7999999999999998, 2.3]]\n\n// Perform element-wise subtraction of two FloatsMatrices\nconsole.log(a.sub(b).data);\n// [[0.9, 1.1, 1.3],\n//  [1.6, 1.7999999999999998, 2.3]]\n\n// Perform element-wise multiplication of two FloatsMatrices\nconsole.log(a.mul(b).data);\n// [[0.08000000000000002, 0.18000000000000002, 0.3],\n//  [0.55, 0.72, 1.1199999999999999]]\n\n// Perform element-wise division of two FloatsMatrices\nconsole.log(b.div(a).data);\n// [[8, 4.5, 3.3333333333333335],\n//  [2.2, 2, 2.285714285714286]]\n\n// Transposes the FloatsMatrix. That is it exchanges the rows and columns.\n// a.tranposed() will return the result of performing the operations\nb.transpose();\nconsole.log(b.data);\n// [[0.8, 1.1],\n//  [0.9, 1.2],\n//  [1, 1.6]]\n\n// Performs dot product of the two FloatsMatrices\nconsole.log(a.dot(b).data);\n// [[0.56, 0.83],\n//  [1.6400000000000001, 2.3899999999999997]]\n\n// Return the addition or product of the FloatsMatrices\nconsole.log(a.sum()); // 2.4000000000000004\nconsole.log(b.product()); // 1.5206400000000002\n\n// Efficiently perform in-place element-wise scaled addition of two FloatsMatrices\na.scaledAdd(2, b.transposed());\nconsole.log(a.data);\n// [[1.7000000000000002, 2, 2.3],\n//  [2.7, 3, 3.9000000000000004]]\n")),(0,r.kt)("h2",{id:"statistical-methods"},"Statistical Methods"),(0,r.kt)("p",null,"Methods to perform basic statistical operations."),(0,r.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,r.kt)("div",{parentName:"div",className:"admonition-heading"},(0,r.kt)("h5",{parentName:"div"},(0,r.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,r.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,r.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,r.kt)("div",{parentName:"div",className:"admonition-content"},(0,r.kt)("p",{parentName:"div"},"All of these methods has a ",(0,r.kt)("inlineCode",{parentName:"p"},"R"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"C")," and normal variant. ",(0,r.kt)("inlineCode",{parentName:"p"},"R")," variant returns a\nFloatsVector by applying the operation on each row. ",(0,r.kt)("inlineCode",{parentName:"p"},"C")," variant returns a\nFloatsVector by applying the operation on each column."))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsMatrix([\n  [0.1, 0.2, 0.3],\n  [0.4, 0.5, 0.6],\n]);\n\n// Return the minimum element in the array\nconsole.log(a.min()); // 0.1\n\n// Return the minimum element in each row\nconsole.log(a.maxR().data); // [0.3, 0.6]\n\n// Returns the mean of each column\nconsole.log(a.meanC().data); // [0.25, 0.35, 0.44999999999999996]\n\n// Return the standard deviation in each row\nconsole.log(a.stdR(0).data); // [0.0816496580927726, 0.08164965809277258]\n\n// Return the variance of the array\nconsole.log(a.varC(1).data); // [0.04500000000000001, 0.045000000000000005, 0.045000000000000005]\n")))}m.isMDXComponent=!0}}]);