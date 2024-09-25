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

  expect(response.length).toBe(2);

  expect(response[0][0]?.from.endsWith("c.js")).toBeTruthy()
  expect(response[0][0]?.to.endsWith("b.js")).toBeTruthy()
  expect(response[0][1]?.from.endsWith("b.js")).toBeTruthy()
  expect(response[0][1]?.to.endsWith("utils.js")).toBeTruthy()

  expect(response[1][0]?.from.endsWith("c.js")).toBeDefined()
  expect(response[1][0]?.to.endsWith("a.js")).toBeDefined()
  expect(response[1][1]?.from.endsWith("a.js")).toBeDefined()
  expect(response[1][1]?.to.endsWith("utils.js")).toBeDefined()
 
})

test('The dependency file list of the specified file should be obtained normally.', () => {
  const cwd = path.resolve(dirname(__filename),"features","alias");
  const response = checkDependencies(path.join(cwd,"c.js"),{
    alias:{
      "@":[cwd]
    },
    cwd,
  })

  expect(response.length).toBe(2);

  expect(response[0][0]?.from.endsWith("c.js")).toBeTruthy()
  expect(response[0][0]?.to.endsWith("b.js")).toBeTruthy()
  expect(response[0][1]?.from.endsWith("b.js")).toBeTruthy()
  expect(response[0][1]?.to.endsWith("utils.js")).toBeTruthy()

  expect(response[1][0]?.from.endsWith("c.js")).toBeDefined()
  expect(response[1][0]?.to.endsWith("a.js")).toBeDefined()
  expect(response[1][1]?.from.endsWith("a.js")).toBeDefined()
  expect(response[1][1]?.to.endsWith("utils.js")).toBeDefined()


})