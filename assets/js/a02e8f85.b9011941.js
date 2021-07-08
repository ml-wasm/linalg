(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[959],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return d},kt:function(){return m}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var i=r.createContext({}),c=function(e){var t=r.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},d=function(e){var t=c(e.components);return r.createElement(i.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,i=e.parentName,d=l(e,["components","mdxType","originalType","parentName"]),u=c(n),m=o,g=u["".concat(i,".").concat(m)]||u[m]||p[m]||a;return n?r.createElement(g,s(s({ref:t},d),{},{components:n})):r.createElement(g,s({ref:t},d))}));function m(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,s=new Array(a);s[0]=u;var l={};for(var i in t)hasOwnProperty.call(t,i)&&(l[i]=t[i]);l.originalType=e,l.mdxType="string"==typeof e?e:o,s[1]=l;for(var c=2;c<a;c++)s[c]=n[c];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},790:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return i},metadata:function(){return c},toc:function(){return d},default:function(){return u}});var r=n(2122),o=n(9756),a=(n(7294),n(3905)),s=["components"],l={title:"IntegersVector"},i=void 0,c={unversionedId:"Vectors/IntegersVector",id:"Vectors/IntegersVector",isDocsHomePage:!1,title:"IntegersVector",description:"IntegersVector is an one dimensional array or a vector of 32-bit integers.",source:"@site/docs/02-Vectors/02-IntegersVector.md",sourceDirName:"02-Vectors",slug:"/Vectors/IntegersVector",permalink:"/linalg/Vectors/IntegersVector",version:"current",sidebarPosition:2,frontMatter:{title:"IntegersVector"},sidebar:"tutorialSidebar",previous:{title:"Vectors",permalink:"/linalg/Vectors/index"},next:{title:"FloatsVector",permalink:"/linalg/Vectors/FloatsVector"}},d=[{value:"Constructors Methods",id:"constructors-methods",children:[]},{value:"Interop Methods",id:"interop-methods",children:[]},{value:"Utility Methods",id:"utility-methods",children:[]},{value:"Math Methods",id:"math-methods",children:[]},{value:"Statistical Methods",id:"statistical-methods",children:[]}],p={toc:d};function u(e){var t=e.components,n=(0,o.Z)(e,s);return(0,a.kt)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"IntegersVector is an one dimensional array or a vector of 32-bit integers."),(0,a.kt)("p",null,"The following scripts assume that you have imported the ",(0,a.kt)("inlineCode",{parentName:"p"},"IntegersVector")," object\nfrom the package and set up the threads as explained in ",(0,a.kt)("a",{parentName:"p",href:"../"},"getting started"),"."),(0,a.kt)("h2",{id:"constructors-methods"},"Constructors Methods"),(0,a.kt)("p",null,"These methods are used to create new ",(0,a.kt)("inlineCode",{parentName:"p"},"IntegerVector"),"s."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-js"},"// Create an IntegersVector from a given JavaScript array\nconst a = new IntegersVector([1, 2, 3, 4, 5]);\nconsole.log(a.data); // [1, 2, 3, 4, 5]\n\n// Create an IntegersVector filled with zeros of the specified length\nconst b = IntegersVector.newWithZeros(5);\nconsole.log(b.data); // [0, 0, 0, 0, 0]\n\n// Create an IntegersVector filled with ones of the specified length\nconst c = IntegersVector.newWithOnes(5);\nconsole.log(c.data); // [1, 1, 1, 1, 1]\n\n// Create an IntegersVector filled with given element of the specified length\nconst d = IntegersVector.newWithElement(5, 2);\nconsole.log(d.data); // [2, 2, 2, 2, 2]\n\n// Create an IntegersVector of specified length calling the given function\n// without any parameters at every element\nconst e = IntegersVector.newWithSimpleFunc(5, () =>\n  Math.floor(Math.random() * 10)\n);\nconsole.log(e.data); // [5, 2, 3, 8, 1]\n\n// Create an IntegersVector of specified length calling the given function with\n// the index as the only parameter for every element\nconst f = IntegersVector.newWithFunc(5, i => i * i);\nconsole.log(f.data); // [0, 1, 4, 9, 16]\n")),(0,a.kt)("h2",{id:"interop-methods"},"Interop Methods"),(0,a.kt)("p",null,"Some handy methods to work with the array."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-js"},'const a = new IntegersVector([1, 2, 3]);\n\n// Both toJSON and data return a JavaScript array representation of the\n// IntegersVector\nconsole.log(a.toJSON()); // [1, 2, 3]\nconsole.log(a.data); // [1, 2, 3]\n\n// This returns the data and metadata about the IntegersVector\nconsole.log(a.toString());\n// "[1, 2, 3], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1"\n\n// It returns clone of the IntegersVector\nconst b = a.clone();\nconsole.log(b.data); // [1, 2, 3]\n')),(0,a.kt)("h2",{id:"utility-methods"},"Utility Methods"),(0,a.kt)("p",null,"Basic getter and setters."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-js"},"const x = new IntegersVector([1, 2, 3]);\n\n// Get the length of the array\nconsole.log(x.len()); // 3\n\n// Get the shape of the array\nconsole.log(x.shape()); // [3]\n\n// Set the given value at the specified index\nx.set(1, 7);\n\n// Get the value at the specified index\nconsole.log(x.get(1)); // 7\n\n// Swap the values at the specified indices\nx.swap(0, 2);\nconsole.log(x.data); // [3, 7, 1]\n")),(0,a.kt)("p",null,"More complex methods used to manipulate the ",(0,a.kt)("inlineCode",{parentName:"p"},"IntegersVector"),'. Each of these\nmethods has two versions. The "pure" version returns the result of performing\nthe operation while the "impure" version actually changes the array.'),(0,a.kt)("table",null,(0,a.kt)("thead",{parentName:"table"},(0,a.kt)("tr",{parentName:"thead"},(0,a.kt)("th",{parentName:"tr",align:null},"Pure"),(0,a.kt)("th",{parentName:"tr",align:null},"Impure"))),(0,a.kt)("tbody",{parentName:"table"},(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:null},"reverse"),(0,a.kt)("td",{parentName:"tr",align:null},"reversed")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:null},"append"),(0,a.kt)("td",{parentName:"tr",align:null},"appended")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:null},"extend"),(0,a.kt)("td",{parentName:"tr",align:null},"extended")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:null},"insert"),(0,a.kt)("td",{parentName:"tr",align:null},"inserted")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:null},"splice"),(0,a.kt)("td",{parentName:"tr",align:null},"spliced")))),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-js"},"const a = new IntegersVector([1, 2, 3]);\nconst b = new IntegersVector([4, 5, 6]);\n\n// Reverse the IntegersVector\nconsole.log(a.reversed().data);\n// [3, 2, 1]\n\n// Append an element to the IntegersVector\nconsole.log(a.appended(7).data);\n// [1, 2, 3, 7]\n\n// Extend the IntegersVector with another\nconsole.log(a.extended(b).data);\n// [1, 2, 3, 4, 5, 6]\n\n// Insert the given element at the specified index\nconsole.log(a.inserted(1, 7).data);\n// [1, 7, 2, 3]\n\n// Removes an element from the specified index\nconst [spliced, element] = a.spliced(1);\nconsole.log(spliced.data, element);\n// [1, 3] 2\n")),(0,a.kt)("h2",{id:"math-methods"},"Math Methods"),(0,a.kt)("p",null,"Methods to perform simple mathematical operations on the array."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-js"},"const a = new IntegersVector([1, 2, 3]);\nconst b = new IntegersVector([4, 5, 6]);\n\n// Perform element-wise addition of two IntegersVectors\nconsole.log(a.add(b).data); // [5, 7, 9]\n\n// Perform element-wise subtraction of two IntegersVectors\nconsole.log(a.sub(b).data); // [-3, -3, -3]\n\n// Perform element-wise multiplication of two IntegersVectors\nconsole.log(a.mul(b).data); // [4, 10, 18]\n\n// Perform element-wise division of two IntegersVectors\nconsole.log(b.div(a).data); // [4, 2, 2]\n\n// Return the addition or product of the IntegersVector\nconsole.log(a.sum()); // 6\nconsole.log(b.product()); // 120\n\n// Efficiently perform in-place element-wise scaled addition of two IntegersVectors\na.scaledAdd(2, b);\nconsole.log(a.data); // [9, 12, 15]\n")),(0,a.kt)("h2",{id:"statistical-methods"},"Statistical Methods"),(0,a.kt)("p",null,"Methods to perform basic statistical operations."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-js"},"const a = new IntegersVector([1, 2, 3]);\n\n// Return the minimum element in the array\nconsole.log(a.min()); // 1\n\n// Return the minimum element in the array\nconsole.log(a.max()); // 3\n\n// Return the mean of all the elements in the array\nconsole.log(a.mean()); // 2\n")))}u.isMDXComponent=!0}}]);