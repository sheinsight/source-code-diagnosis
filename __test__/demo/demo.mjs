import ava_test from "ava";
import a, {checkDangerStrings} from "../../index.js";
import { performance } from "node:perf_hooks";

ava_test(`should to has demo`, (t) => {
  const start = performance.now();
  console.log(a);
  // const res = checkDangerStrings("", {
  //   cwd: __dirname,
  //   concurrency: 1,
  // });

//   res.sort((a, b) => {
//     return +a.compat.support.chrome > +b.compat.support.chrome ? -1 : 1;
//   });

//   console.log(res[0].compat,res[0].compat.support.chrome);

//   console.log(`
//    您的代码最低可以兼容到 Chrome ${res[0].compat.support.chrome} , Firefox ${res[0].compat.support.firefox} , Safari ${res[0].compat.support.safari} , Edge ${res[0].compat.support.edge} , IE ${res[0].compat.support.ie} 
//    您使用的最新的代码特性为 ${res[0].compat.name} ${res[0].compat.description} ${res[0].compat.tags.join(",")}
   
//    以下是您的代码片段

//     ${res[0].codeSeg}

//   他位于 ${res[0].start}:${res[0].end} 
// `);

//   let mapper = {};

  

//   res.map(item => {
//     if (mapper[item.compat.name]) {
//       mapper[item.compat.name].push(item);
//     }else {
//       mapper[item.compat.name] = [item];
//     }
//   })

//   for (const iterator in mapper) {
//    console.log(iterator, mapper[iterator].length,mapper[iterator][0].compat.support.chrome); 
//   }

  const end = performance.now();

  console.log(`cost: ${end - start}ms`);

  t.truthy(true);
});