import { expect, test, } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependencies} from '../../index.js'



const __filename = fileURLToPath(import.meta.url);


test('The dependency file list of the specified file should be obtained normally.', () => {
  const cwd = path.resolve(dirname(__filename),"features","normal");
  const response = checkDependencies(path.join(cwd,"c.js"),{
    cwd,
  })

  let expectFiles = ["a.js","b.js"]

  expect(expectFiles.length).toBe(response.length);

  for (const item of response) {
    let res = expectFiles.some((name)=>item.endsWith(name))
    expect(res).toBeTruthy()
  }
})

test('The dependency file list of the specified file should be obtained normally.', () => {
  const cwd = path.resolve(dirname(__filename),"features","alias");
  const response = checkDependencies(path.join(cwd,"c.js"),{
    alias:{
      "@":[cwd]
    },
    cwd,
  })

  let expectFiles = ["a.js","b.js"]

  expect(expectFiles.length).toBe(response.length);
})