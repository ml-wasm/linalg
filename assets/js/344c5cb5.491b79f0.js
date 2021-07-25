(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[137],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return p},kt:function(){return m}});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),c=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=c(e.components);return r.createElement(s.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),d=c(n),m=a,f=d["".concat(s,".").concat(m)]||d[m]||u[m]||o;return n?r.createElement(f,i(i({ref:t},p),{},{components:n})):r.createElement(f,i({ref:t},p))}));function m(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:a,i[1]=l;for(var c=2;c<o;c++)i[c]=n[c];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},2560:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return l},contentTitle:function(){return s},metadata:function(){return c},toc:function(){return p},default:function(){return d}});var r=n(2122),a=n(9756),o=(n(7294),n(3905)),i=["components"],l={slug:"/",title:"Getting Started"},s=void 0,c={unversionedId:"Getting Started",id:"Getting Started",isDocsHomePage:!1,title:"Getting Started",description:"The goal of this project is to provide a **fast, easy-to-use linear algebra",source:"@site/docs/01-Getting Started.md",sourceDirName:".",slug:"/",permalink:"/linalg/",editUrl:"https://github.com/ml-wasm/linalg/edit/master/docs/docs/01-Getting Started.md",version:"current",sidebarPosition:1,frontMatter:{slug:"/",title:"Getting Started"},sidebar:"tutorialSidebar",next:{title:"Vectors",permalink:"/linalg/Vectors/index"}},p=[{value:"Installing the package",id:"installing-the-package",children:[]},{value:"How to use",id:"how-to-use",children:[]}],u={toc:p};function d(e){var t=e.components,n=(0,a.Z)(e,i);return(0,o.kt)("wrapper",(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"The goal of this project is to provide a ",(0,o.kt)("strong",{parentName:"p"},"fast, easy-to-use linear algebra\nJavascript library")," powered by WebAssembly, written in Rust."),(0,o.kt)("p",null,"This project is a part of a larger project called ",(0,o.kt)("a",{parentName:"p",href:"https://www.github.com/wasml"},"wasml"),",\nwhich aims to provide a complete machine learning ecosystem in JavaScript."),(0,o.kt)("h2",{id:"installing-the-package"},"Installing the package"),(0,o.kt)("p",null,(0,o.kt)("em",{parentName:"p"},"Coming Soon")),(0,o.kt)("h2",{id:"how-to-use"},"How to use"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-js"},"import init, {\n  initThreadPool,\n  Integers1d,\n  Floats1d,\n  Strings1d,\n  Integers2d,\n  Floats2d,\n  Strings2d,\n} from '@wasml/linalg';\n\n(async () => {\n  // This init function sets up everything you need to use this library\n  await init();\n\n  // This sets up the concurrency\n  await initThreadPool(navigator.hardwareConcurrency);\n\n  // All your code goes here...\n})();\n")),(0,o.kt)("div",{className:"admonition admonition-warning alert alert--danger"},(0,o.kt)("div",{parentName:"div",className:"admonition-heading"},(0,o.kt)("h5",{parentName:"div"},(0,o.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,o.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"12",height:"16",viewBox:"0 0 12 16"},(0,o.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M5.05.31c.81 2.17.41 3.38-.52 4.31C3.55 5.67 1.98 6.45.9 7.98c-1.45 2.05-1.7 6.53 3.53 7.7-2.2-1.16-2.67-4.52-.3-6.61-.61 2.03.53 3.33 1.94 2.86 1.39-.47 2.3.53 2.27 1.67-.02.78-.31 1.44-1.13 1.81 3.42-.59 4.78-3.42 4.78-5.56 0-2.84-2.53-3.22-1.25-5.61-1.52.13-2.03 1.13-1.89 2.75.09 1.08-1.02 1.8-1.86 1.33-.67-.41-.66-1.19-.06-1.78C8.18 5.31 8.68 2.45 5.05.32L5.03.3l.02.01z"}))),"Proper error handling not implemented yet")),(0,o.kt)("div",{parentName:"div",className:"admonition-content"},(0,o.kt)("p",{parentName:"div"},'If you are familiar with Rust, currently all the functions just "panic". This\nwill be fixed in the future.'))),(0,o.kt)("p",null,"For more information on how to work with one dimensional arrays or vectors go\n",(0,o.kt)("a",{parentName:"p",href:"Vectors/index"},"here"),". For two dimensional arrays or matrices go\n",(0,o.kt)("a",{parentName:"p",href:"Vectors/index"},"here"),"."))}d.isMDXComponent=!0}}]);