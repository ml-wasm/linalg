(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[63],{3905:function(e,t,r){"use strict";r.d(t,{Zo:function(){return p},kt:function(){return d}});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},p=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),f=l(r),d=o,m=f["".concat(c,".").concat(d)]||f[d]||u[d]||a;return r?n.createElement(m,i(i({ref:t},p),{},{components:r})):n.createElement(m,i({ref:t},p))}));function d(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=f;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:o,i[1]=s;for(var l=2;l<a;l++)i[l]=r[l];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},5547:function(e,t,r){"use strict";r.r(t),r.d(t,{frontMatter:function(){return s},contentTitle:function(){return c},metadata:function(){return l},toc:function(){return p},default:function(){return f}});var n=r(2122),o=r(9756),a=(r(7294),r(3905)),i=["components"],s={title:"Vectors"},c=void 0,l={unversionedId:"Vectors/index",id:"Vectors/index",isDocsHomePage:!1,title:"Vectors",description:"Currently there are three supported data types for one dimensional arrays or",source:"@site/docs/02-Vectors/01-index.md",sourceDirName:"02-Vectors",slug:"/Vectors/index",permalink:"/linalg/Vectors/index",editUrl:"https://github.com/wasml/linalg/edit/master/docs/docs/02-Vectors/01-index.md",version:"current",sidebarPosition:1,frontMatter:{title:"Vectors"},sidebar:"tutorialSidebar",previous:{title:"Getting Started",permalink:"/linalg/"},next:{title:"IntegersVector",permalink:"/linalg/Vectors/IntegersVector"}},p=[],u={toc:p};function f(e){var t=e.components,r=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"Currently there are three supported data types for one dimensional arrays or\nvectors:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"IntegersVector"},"IntegersVector"),": Used to represent a vector of integers. The\nunderlying type used is a 32-bit integer. It supports math operations but\ndoesn't support complex statistical operations."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"FloatsVector"},"FloatsVector"),": Used to represent a vector of floats. The underlying\ntype used is a 64-bit floating point number. It supports math and statistical\noperations."),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("a",{parentName:"li",href:"StringsVector"},"StringsVector"),": Used to represent a vector of strings. As expected, it\ndoesn't support any mathematical or statistical operations.")),(0,a.kt)("p",null,"Other types like booleans, dates, objects will probably be implemented in the\nfuture."))}f.isMDXComponent=!0}}]);