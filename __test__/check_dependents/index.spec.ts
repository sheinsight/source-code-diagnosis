import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependents} from '../../index.js'



const __filename = fileURLToPath(import.meta.url);
 
test('Get which files depend on the specified file', () => {
  const cwd = path.resolve(dirname(__filename),"features","normal");

  const response = checkDependents(path.join(cwd,"utils.js"),{
    alias:{
      "@":[cwd]
    },
    cwd,
  })

  console.log(response);
  

  for (const item of response) {
    console.log("--------------------------------");
    
    for (const item2 of item) {
      console.log(item2.from,item2.to,item2.astNode.loc);
    }
  }

  expect(response.length).toBe(2)
})



test('Get which files depend on the specified file with alias', () => {
  const cwd = path.resolve(dirname(__filename),"features","alias");  
  const response = checkDependents(path.join(cwd,"utils.js"),{
    alias:{
      "@":[cwd]
    },
    cwd,
  })

  expect(response.length).toBe(2)
})
 