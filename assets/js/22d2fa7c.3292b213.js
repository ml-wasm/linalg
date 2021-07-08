(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[528],{3905:function(e,t,r){"use strict";r.d(t,{Zo:function(){return p},kt:function(){return m}});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},p=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),f=l(r),m=a,d=f["".concat(c,".").concat(m)]||f[m]||u[m]||i;return r?n.createElement(d,o(o({ref:t},p),{},{components:r})):n.createElement(d,o({ref:t},p))}));function m(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=f;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:a,o[1]=s;for(var l=2;l<i;l++)o[l]=r[l];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},5085:function(e,t,r){"use strict";r.r(t),r.d(t,{frontMatter:function(){return s},contentTitle:function(){return c},metadata:function(){return l},toc:function(){return p},default:function(){return f}});var n=r(2122),a=r(9756),i=(r(7294),r(3905)),o=["components"],s={title:"Matrices"},c=void 0,l={unversionedId:"Matrices/index",id:"Matrices/index",isDocsHomePage:!1,title:"Matrices",description:"Currently there are three supported data types for two dimensional arrays or",source:"@site/docs/03-Matrices/01-index.md",sourceDirName:"03-Matrices",slug:"/Matrices/index",permalink:"/linalg/Matrices/index",version:"current",sidebarPosition:1,frontMatter:{title:"Matrices"},sidebar:"tutorialSidebar",previous:{title:"StringsVector",permalink:"/linalg/Vectors/StringsVector"},next:{title:"IntegersMatrix",permalink:"/linalg/Matrices/IntegersMatrix"}},p=[],u={toc:p};function f(e){var t=e.components,r=(0,a.Z)(e,o);return(0,i.kt)("wrapper",(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"Currently there are three supported data types for two dimensional arrays or\nmatrices:"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("a",{parentName:"li",href:"IntegersMatrix"},"IntegersMatrix"),": Used to represent a matrix of\nintegers. The underlying type used is a 32-bit integer. It supports math\noperations but doesn't support complex statistical operations."),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("a",{parentName:"li",href:"FloatsMatrix"},"FloatsMatrix"),": Used to represent a matrix of floats. The\nunderlying type used is a 64-bit floating point number. It supports math and\nstatistical operations."),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("a",{parentName:"li",href:"StringsMatrix"},"StringsMatrix"),": Used to represent a matrix of strings.\nAs expected, it doesn't support any mathematical or statistical operations.")),(0,i.kt)("p",null,"Other types like booleans, dates, objects will probably be implemented in the\nfuture."))}f.isMDXComponent=!0}}]);