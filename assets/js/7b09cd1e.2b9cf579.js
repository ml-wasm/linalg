(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[153],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return d},kt:function(){return p}});var o=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);t&&(o=o.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,o)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,o,a=function(e,t){if(null==e)return{};var n,o,a={},r=Object.keys(e);for(o=0;o<r.length;o++)n=r[o],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(o=0;o<r.length;o++)n=r[o],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var c=o.createContext({}),i=function(e){var t=o.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},d=function(e){var t=i(e.components);return o.createElement(c.Provider,{value:t},e.children)},m={inlineCode:"code",wrapper:function(e){var t=e.children;return o.createElement(o.Fragment,{},t)}},h=o.forwardRef((function(e,t){var n=e.components,a=e.mdxType,r=e.originalType,c=e.parentName,d=l(e,["components","mdxType","originalType","parentName"]),h=i(n),p=a,u=h["".concat(c,".").concat(p)]||h[p]||m[p]||r;return n?o.createElement(u,s(s({ref:t},d),{},{components:n})):o.createElement(u,s({ref:t},d))}));function p(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var r=n.length,s=new Array(r);s[0]=h;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l.mdxType="string"==typeof e?e:a,s[1]=l;for(var i=2;i<r;i++)s[i]=n[i];return o.createElement.apply(null,s)}return o.createElement.apply(null,n)}h.displayName="MDXCreateElement"},2042:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return c},metadata:function(){return i},toc:function(){return d},default:function(){return h}});var o=n(2122),a=n(9756),r=(n(7294),n(3905)),s=["components"],l={title:"FloatsVector"},c=void 0,i={unversionedId:"Vectors/FloatsVector",id:"Vectors/FloatsVector",isDocsHomePage:!1,title:"FloatsVector",description:"FloatsVector is an one dimensional array or a vector of 64-bit floats.",source:"@site/docs/02-Vectors/03-FloatsVector.md",sourceDirName:"02-Vectors",slug:"/Vectors/FloatsVector",permalink:"/linalg/Vectors/FloatsVector",editUrl:"https://github.com/ml-wasm/linalg/edit/master/docs/docs/02-Vectors/03-FloatsVector.md",version:"current",sidebarPosition:3,frontMatter:{title:"FloatsVector"},sidebar:"tutorialSidebar",previous:{title:"IntegersVector",permalink:"/linalg/Vectors/IntegersVector"},next:{title:"StringsVector",permalink:"/linalg/Vectors/StringsVector"}},d=[{value:"Constructors Methods",id:"constructors-methods",children:[]},{value:"Interop Methods",id:"interop-methods",children:[]},{value:"Utility Methods",id:"utility-methods",children:[]},{value:"Iteration Methods",id:"iteration-methods",children:[]},{value:"Math Methods",id:"math-methods",children:[]},{value:"Statistical Methods",id:"statistical-methods",children:[]},{value:"Random Methods",id:"random-methods",children:[]}],m={toc:d};function h(e){var t=e.components,n=(0,a.Z)(e,s);return(0,r.kt)("wrapper",(0,o.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"FloatsVector is an one dimensional array or a vector of 64-bit floats."),(0,r.kt)("p",null,"The following scripts assume that you have imported the ",(0,r.kt)("inlineCode",{parentName:"p"},"FloatsVector")," object\nfrom the package and set up the threads as explained in ",(0,r.kt)("a",{parentName:"p",href:"../"},"getting started"),"."),(0,r.kt)("h2",{id:"constructors-methods"},"Constructors Methods"),(0,r.kt)("p",null,"These methods are used to create new ",(0,r.kt)("inlineCode",{parentName:"p"},"FloatsVector"),"s."),(0,r.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,r.kt)("div",{parentName:"div",className:"admonition-heading"},(0,r.kt)("h5",{parentName:"div"},(0,r.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,r.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,r.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,r.kt)("div",{parentName:"div",className:"admonition-content"},(0,r.kt)("p",{parentName:"div"},"Random constructors are in the ",(0,r.kt)("a",{parentName:"p",href:"#random-methods"},"random section"),"."))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"// Create a FloatsVector from a given JavaScript array\nconst a = new FloatsVector([1, 2, 3, 4, 5]);\nconsole.log(a.data); // [1, 2, 3, 4, 5]\n\n// Create a FloatsVector filled with zeros of the specified length\nconst b = FloatsVector.newWithZeros(5);\nconsole.log(b.data); // [0, 0, 0, 0, 0]\n\n// Create a FloatsVector filled with ones of the specified length\nconst c = FloatsVector.newWithOnes(5);\nconsole.log(c.data); // [1, 1, 1, 1, 1]\n\n// Create a FloatsVector filled with given element of the specified length\nconst d = FloatsVector.newWithElement(5, 2);\nconsole.log(d.data); // [2, 2, 2, 2, 2]\n\n// Create a FloatsVector of specified length calling the given function\n// without any parameters at every element\nconst e = FloatsVector.newWithSimpleFunc(3, () => Math.random() * 10);\nconsole.log(e.data); // [1.6528487232156874, 3.3231458607562137, 8.349146447113004]\n\n// Create a FloatsVector of specified length calling the given function with\n// the index as the only parameter for every element\nconst f = FloatsVector.newWithFunc(5, i => i * i);\nconsole.log(f.data); // [0, 1, 4, 9, 16]\n\n// Create a FloatsVector filled with n evenly spaced elements between two points\nconst g = FloatsVector.newWithLinspace(0, 1, 5);\nconsole.log(g.data); // [0, 0.25, 0.5, 0.75, 1]\n\n// Create a FloatsVector with elements from start to step incrementing by step\nconst h = FloatsVector.newWithRange(0, 0.75, 0.25);\nconsole.log(h.data); // [0, 0.25, 0.5]\n\n// Create a FloatsVector with n logarithmically spaced elements from base^start\n// to base^end\nconst i = FloatsVector.newWithLogspace(2, 0, 10, 5);\nconsole.log(i.data); // [2, 4, 8, 16]\n\n// Create a FloatsVector with n geometrically spaced elements from start to end\nconst j = FloatsVector.newWithGeomspace(1, 5, 3);\nconsole.log(j.data); // [1, 1.709975946676697, 2.924017738212866, 5.000000000000001]\n")),(0,r.kt)("h2",{id:"interop-methods"},"Interop Methods"),(0,r.kt)("p",null,"Some handy methods to work with the array."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},'const a = new FloatsVector([1, 2, 3]);\n\n// Both toJSON and data return a JavaScript array representation of the\n// FloatsVector\nconsole.log(a.toJSON()); // [1, 2, 3]\nconsole.log(a.data); // [1, 2, 3]\n\n// This returns the data and metadata about the FloatsVector\nconsole.log(a.toString());\n// "[1, 2, 3], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1"\n\n// It returns clone of the FloatsVector\nconst b = a.clone();\nconsole.log(b.data); // [1, 2, 3]\n')),(0,r.kt)("h2",{id:"utility-methods"},"Utility Methods"),(0,r.kt)("p",null,"Basic getters and setters."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const x = new FloatsVector([1, 2, 3]);\n\n// Get the length of the array\nconsole.log(x.len()); // 3\n\n// Get the shape of the array\nconsole.log(x.shape()); // [3]\n\n// Set the given value at the specified index\nx.set(1, 7);\n\n// Get the value at the specified index\nconsole.log(x.get(1)); // 7\n\n// Swap the values at the specified indices\nx.swap(0, 2);\nconsole.log(x.data); // [3, 7, 1]\n")),(0,r.kt)("p",null,"More complex methods used to manipulate the ",(0,r.kt)("inlineCode",{parentName:"p"},"FloatsVector"),"."),(0,r.kt)("div",{className:"admonition admonition-note alert alert--secondary"},(0,r.kt)("div",{parentName:"div",className:"admonition-heading"},(0,r.kt)("h5",{parentName:"div"},(0,r.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,r.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"14",height:"16",viewBox:"0 0 14 16"},(0,r.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"}))),"note")),(0,r.kt)("div",{parentName:"div",className:"admonition-content"},(0,r.kt)("p",{parentName:"div"},'Each of these methods has two versions. The "pure" version returns the result of\nperforming the operation while the "impure" version actually changes the array.'),(0,r.kt)("p",{parentName:"div"},(0,r.kt)("inlineCode",{parentName:"p"},"append -> appended"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"extend -> extended"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"insert -> inserted"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"splice -> spliced")))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsVector([0.1, 0.2, 0.3]);\nconst b = new FloatsVector([0.4, 0.5, 0.6]);\n\n// Reverse the FloatsVector\nconsole.log(a.reversed().data);\n// [0.3, 0.2, 0.1]\n\n// Append an element to the FloatsVector\nconsole.log(a.appended(0.7).data);\n// [0.1, 0.2, 0.3, 0.7]\n\n// Extend the FloatsVector with another\nconsole.log(a.extended(b).data);\n// [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]\n\n// Insert the given element at the specified index\nconsole.log(a.inserted(1, 0.7).data);\n// [0.1, 0.7, 0.2, 0.3]\n\n// Removes an element from the specified index\nconst [spliced, element] = a.spliced(1);\nconsole.log(spliced.data, element);\n// [0.1, 0.3] 0.2\n")),(0,r.kt)("h2",{id:"iteration-methods"},"Iteration Methods"),(0,r.kt)("p",null,"These methods allow you to perform element-wise operations on the vector."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsVector([0.1, 0.2, 0.3]);\n\nconst b = a.map(x => x * 3);\nconsole.log(b.data); // [0.30000000000000004, 0.6000000000000001, 0.8999999999999999]\n\na.forEach(x => console.log(x));\n// 0.1\n// 0.2\n// 0.3\n\na.transform(x => x * x);\nconsole.log(a.data); // [0.010000000000000002, 0.04000000000000001, 0.09]\n")),(0,r.kt)("h2",{id:"math-methods"},"Math Methods"),(0,r.kt)("p",null,"Methods to perform simple mathematical operations on the array."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsVector([0.1, 0.2, 0.3]);\nconst b = new FloatsVector([0.4, 0.5, 0.6]);\n\n// Perform element-wise addition of two FloatsVectors\nconsole.log(a.add(b).data); // [0.5, 0.7, 0.8999999999999999]\n\n// Perform element-wise subtraction of two FloatsVectors\nconsole.log(a.sub(b).data); // [-0.30000000000000004, -0.3, -0.3]\n\n// Perform element-wise multiplication of two FloatsVectors\nconsole.log(a.mul(b).data); // [0.04000000000000001, 0.1, 0.18]\n\n// Perform element-wise division of two FloatsVectors\nconsole.log(b.div(a).data); // [4, 2.5, 2]\n\n// Return the addition or product of the FloatsVector\nconsole.log(a.sum()); // 0.6000000000000001\nconsole.log(b.product()); // 0.12\n\n// Efficiently perform in-place element-wise scaled addition of two FloatsVectors\na.scaledAdd(2, b);\nconsole.log(a.data); // [0.9, 1.2, 1.5]\n")),(0,r.kt)("h2",{id:"statistical-methods"},"Statistical Methods"),(0,r.kt)("p",null,"Methods to perform basic statistical operations."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = new FloatsVector([0.1, 0.2, 0.3]);\n\n// Return the minimum element in the array\nconsole.log(a.min()); // 0.1\n\n// Return the minimum element in the array\nconsole.log(a.max()); // 0.3\n\n// Return the mean of all the elements in the array\nconsole.log(a.mean()); // 0.20000000000000004\n\n// Return the standard deviation of the array\nconsole.log(a.std(0)); // 0.0816496580927726\n\n// Return the variance of the array\nconsole.log(a.var(1)); // 0.009999999999999997\n")),(0,r.kt)("h2",{id:"random-methods"},"Random Methods"),(0,r.kt)("p",null,"Methods that involve randomness."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-js"},"const a = FloatsVector.newWithRandom(5);\nconsole.log(a.data);\n// [0.34099948653292644, 0.5917974289682977, 0.6106158620616635, 0.23569474303579374, 0.5282766210914832]\n\nconst b = FloatsVector.newWithRandomBeta(5, 2, 5);\nconsole.log(b.data);\n// [0.23096671212947778, 0.3534113458857726, 0.34475362808214677, 0.3933605194453189, 0.4257499695718641]\n\nconst c = FloatsVector.newWithRandomCauchy(5, 2, 5);\nconsole.log(c.data);\n// [-10.259169814528763, 2.2570224421228415, -0.5143187859414864, -3.566078787268461, 1.8881158135056801]\n\nconst d = FloatsVector.newWithRandomChiSquared(5, 11);\nconsole.log(d.data);\n// [12.540935152190382, 11.870956625621746, 17.77216830488515, 10.783752732928962, 10.664649797233045]\n\nconst e = FloatsVector.newWithRandomExp(5, 10);\nconsole.log(e.data);\n// [0.1582298816595742, 0.15597280990902898, 0.026172146240797518, 0.033239085350025364, 0.07938556756009543]\n\nconst f = FloatsVector.newWithRandomExp1(5);\nconsole.log(f.data);\n// [0.3786935465784936, 0.324525176947993, 1.0327496064187238, 0.08707228389123019, 1.4594054734003976]\n\nconst g = FloatsVector.newWithRandomFisher(5, 2, 32);\nconsole.log(g.data);\n// [0.3074038581481312, 2.1847693566847393, 0.2276554651866491, 0.1163362059260453, 0.6359674157651448]\n\nconst h = FloatsVector.newWithRandomGamma(5, 2, 5);\nconsole.log(h.data);\n// [13.64534682579972, 13.18225900805788, 10.813782133484146, 12.138039285872178, 5.944892095380422]\n\nconst i = FloatsVector.newWithRandomInverseGaussian(5, 2, 5);\nconsole.log(i.data);\n// [2.271010640745854, 2.033124138574522, 3.913079484773854, 1.505314818081629, 0.6226851876937998]\n\nconst j = FloatsVector.newWithRandomLogNormal(5, 2, 3);\nconsole.log(j.data);\n// [4.657210763636486, 66.24632860255005, 20.613278570556297, 10492.633098375172, 39.12513559580575]\n\nconst k = FloatsVector.newWithRandomNormal(5, 2, 3);\nconsole.log(k.data);\n// [0.39029435201635665, 3.27261609644719, 5.0629281791580505, 0.27925119369574114, 5.240239044741968]\n\nconst l = FloatsVector.newWithRandomNormalInverseGaussian(5, 3, 2);\nconsole.log(l.data);\n// [1.2097230563868009, 0.9542971768376568, 0.12535857990674565, 0.9559384049858257, 1.0198942781079443]\n\nconst m = FloatsVector.newWithRandomOpen01(5);\nconsole.log(m.data);\n// [0.14561860008297822, 0.2078389740691543, 0.518045179245913, 0.5737861058411688, 0.7382741221752666]\n\nconst n = FloatsVector.newWithRandomOpenClosed01(5);\nconsole.log(n.data);\n// [0.9188955843628368, 0.319868332534298, 0.44071331349815046, 0.3528910186300348, 0.8702363940864016]\n\nconst o = FloatsVector.newWithRandomPERT(5, 0, 5, 2.5);\nconsole.log(o.data);\n// [0.22585318866399032, 3.600397861643679, 0.9194109223130829, 2.2344645171396156, 1.9741550812422148]\n\nconst p = FloatsVector.newWithRandomPERTwithShape(5, 0, 5, 2.5, 3);\nconsole.log(p.data);\n// [3.9604520876027465, 4.088791843088483, 1.5617571788323033, 3.436157392852803, 2.0772133687604]\n\nconst r = FloatsVector.newWithRandomPareto(5, 1, 2);\nconsole.log(r.data);\n// [1.1307878092838028, 1.0069139977625745, 1.1074903396292302, 1.060290197461119, 1.0271930755716328]\n\nconst s = FloatsVector.newWithRandomPoisson(5, 2.0);\nconsole.log(s.data);\n// [1, 2, 1, 1, 1]\n\nconst t = FloatsVector.newWithRandomStandardNormal(5);\nconsole.log(t.data);\n// [-1.174592862530223, -1.6549897461536043, -0.39595930094841264, 0.6132371623244752, -0.7785965936342404]\n\nconst u = FloatsVector.newWithRandomStudentT(5, 11);\nconsole.log(u.data);\n// [-0.5892888945409621, 0.058722742800470386, 0.2806955947923983, 1.6907601263659682, 1.6528920846027841]\n\nconst v = FloatsVector.newWithRandomTriangular(5, 0, 5, 2.5);\nconsole.log(v.data);\n// [1.1120111694995478, 4.0989948710773065, 2.802263571151345, 1.0294591379745555, 0.9801948864580505]\n\nconst w = FloatsVector.newWithRandomUniform(5, -5, 5);\nconsole.log(w.data);\n// [1.1120111694995478, 4.0989948710773065, 2.802263571151345, 1.0294591379745555, 0.9801948864580505]\n\nconst x = FloatsVector.newWithRandomWeibull(5, 1, 10);\nconsole.log(x.data);\n// [0.6117699677190211, 0.8563173903147892, 0.8959745267683338, 1.1108808965201318, 1.092721654565621]\n\nconst aa = a.sample(3);\nconsole.log(aa.data);\n// [0.02612549959263477, 0.31168488926149907, 0.77341724202506]\n")))}h.isMDXComponent=!0}}]);