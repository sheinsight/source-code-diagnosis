import { expect, test } from 'vitest'
import { fileURLToPath } from "node:url";
import path, { dirname } from "node:path";
import { checkDependents } from '../../index.js'

const __filename = fileURLToPath(import.meta.url);
 
test('should return 1 result when module member used in tsx with use-in-jsx-element', () => {
  const cwd = path.resolve(dirname(__filename),"features","normal");
  const response = checkDependents(`${cwd}/utils.js`,{
    alias:{
      "@":[cwd]
    },
    cwd,
  })

  expect(response.length).toBe(2)
})



test('should return 1 result when module member used in tsx with use-in-jsx-element1', () => {
  const cwd = path.resolve(dirname(__filename),"features","alias");  
  const response = checkDependents(`${cwd}/utils.js`,{
    alias:{
      "@":[cwd]
    },
    cwd,
  })
  expect(response.length).toBe(2)
})
